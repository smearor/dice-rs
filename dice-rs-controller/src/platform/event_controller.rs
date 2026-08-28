use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Duration;

use dice_rs::service::dice::Dice;
use dice_rs::service::dice::DiceEvent;
use dice_rs::service::manager::DiceManager;
use tokio::sync::Notify;
use tokio::sync::broadcast::error::RecvError;
use tracing::debug;

use crate::platform::ui_update::UiUpdate;

/// Interval for periodic battery level refresh.
const BATTERY_REFRESH_INTERVAL_SECS: u64 = 30;

/// Interval for battery level refresh while charging.
const BATTERY_CHARGING_REFRESH_INTERVAL_SECS: u64 = 1;

/// Delay before first reconnect attempt after a disconnect.
const RECONNECT_INITIAL_DELAY_SECS: u64 = 2;

/// Bridges async dice events into a channel for the UI to consume.
///
/// The event loop runs on a `tokio::spawn` task so it never blocks the GTK
/// main thread. UI updates are marshaled back via a `std::sync::mpsc` channel
/// that the caller polls on the GTK side.
pub struct EventController {
    dice: Dice,
    manager: Arc<DiceManager>,
}

impl EventController {
    /// Create a new event controller.
    pub fn new(dice: Dice, manager: Arc<DiceManager>) -> Self {
        Self { dice, manager }
    }

    /// Start listening for dice events and sending UI updates via `sender`.
    pub fn start(&self, sender: mpsc::Sender<UiUpdate>) {
        // Tokio task: event loop - runs entirely off the GTK main thread.
        let dice = self.dice.clone();
        let dice_name = dice.name().to_string();
        let manager = self.manager.clone();
        let event_sender = sender.clone();
        let charging_flag = Arc::new(AtomicBool::new(false));
        let charging_flag_for_battery = charging_flag.clone();
        let charging_notify = Arc::new(Notify::new());
        let charging_notify_for_battery = charging_notify.clone();
        tokio::spawn(async move {
            let mut receiver = dice.subscribe();

            loop {
                match receiver.recv().await {
                    Ok(DiceEvent::RollStart) => {
                        if !charging_flag.load(Ordering::Relaxed) {
                            let _ = event_sender.send(UiUpdate::Rolling);
                        }
                    }
                    Ok(DiceEvent::Stable { face, acceleration }) => {
                        if !charging_flag.load(Ordering::Relaxed) {
                            let _ = event_sender.send(UiUpdate::Stable { face, acceleration });
                        }
                    }
                    Ok(DiceEvent::TiltStable { face, acceleration }) => {
                        if !charging_flag.load(Ordering::Relaxed) {
                            let _ = event_sender.send(UiUpdate::TiltStable { face, acceleration });
                        }
                    }
                    Ok(DiceEvent::FakeStable { face, acceleration }) => {
                        if !charging_flag.load(Ordering::Relaxed) {
                            let _ = event_sender.send(UiUpdate::FakeStable { face, acceleration });
                        }
                    }
                    Ok(DiceEvent::MoveStable { face, acceleration }) => {
                        if !charging_flag.load(Ordering::Relaxed) {
                            let _ = event_sender.send(UiUpdate::MoveStable { face, acceleration });
                        }
                    }
                    Ok(DiceEvent::Charging { state }) => {
                        let charging = bool::from(state);
                        charging_flag.store(charging, Ordering::Relaxed);
                        if charging {
                            charging_notify.notify_one();
                        }
                        let _ = event_sender.send(UiUpdate::Charging { state });
                    }
                    Ok(DiceEvent::Tap) => {
                        let _ = event_sender.send(UiUpdate::Tap);
                    }
                    Ok(DiceEvent::DoubleTap) => {
                        let _ = event_sender.send(UiUpdate::DoubleTap);
                    }
                    Ok(DiceEvent::Disconnected) => {
                        let _ = event_sender.send(UiUpdate::Disconnected);

                        // Auto-reconnect on the tokio threadpool.
                        tokio::time::sleep(Duration::from_secs(RECONNECT_INITIAL_DELAY_SECS)).await;
                        match manager.reconnect(&dice).await {
                            Ok(()) => {
                                debug!(device = %dice_name, "reconnected successfully, resuming event loop");
                                receiver = dice.subscribe();
                            }
                            Err(error) => {
                                debug!(device = %dice_name, error = %error, "reconnect failed permanently");
                                break;
                            }
                        }
                    }
                    Err(RecvError::Lagged(skipped)) => {
                        debug!(device = %dice_name, skipped, "event receiver lagged, continuing");
                    }
                    Err(RecvError::Closed) => {
                        debug!(device = %dice_name, "event channel closed, stopping event controller");
                        break;
                    }
                }
            }
        });

        // Tokio task: periodic battery refresh - fully off the GTK main thread.
        // Polls more frequently while the dice is charging.
        let battery_dice = self.dice.clone();
        let battery_name = self.dice.name().to_string();
        let battery_sender = sender;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            match battery_dice.get_battery_level().await {
                Ok(level) => {
                    let _ = battery_sender.send(UiUpdate::BatteryLevel(level));
                }
                Err(error) => debug!(device = %battery_name, error = %error, "initial battery fetch failed"),
            }

            loop {
                // Use dice.charging_state() as ground truth - the broadcast event
                // can be missed (lag), but the AtomicU8 in DiceInner is always
                // updated by the notification task.
                let state = battery_dice.charging_state();
                let charging = bool::from(state);
                let prev = charging_flag_for_battery.swap(charging, Ordering::Relaxed);
                if charging != prev {
                    debug!(device = %battery_name, charging, prev, "charging state sync via battery task");
                    let _ = battery_sender.send(UiUpdate::Charging { state });
                }

                let interval = if charging {
                    BATTERY_CHARGING_REFRESH_INTERVAL_SECS
                } else {
                    BATTERY_REFRESH_INTERVAL_SECS
                };
                // When not charging, wake early if charging starts.
                if charging {
                    tokio::time::sleep(Duration::from_secs(interval)).await;
                } else {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_secs(interval)) => {}
                        _ = charging_notify_for_battery.notified() => {}
                    }
                }
                match battery_dice.is_connected().await {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => {
                        debug!(device = %battery_name, error = %error, "battery refresh: is_connected failed");
                        continue;
                    }
                }
                match battery_dice.get_battery_level().await {
                    Ok(level) => {
                        debug!(device = %battery_name, charging, level = level.get(), "battery refresh succeeded");
                        let _ = battery_sender.send(UiUpdate::BatteryLevel(level));
                    }
                    Err(error) => debug!(device = %battery_name, error = %error, charging, "battery refresh failed"),
                }
            }
        });
    }
}
