use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use dice_rs::model::acceleration::Acceleration;
use dice_rs::model::face::FaceValue;
use dice_rs::model::stability_descriptor::StabilityDescriptor;
use dice_rs::service::dice::Dice;
use dice_rs::service::dice::DiceEvent;
use dice_rs::service::manager::DiceManager;
use gtk4::glib;
use tokio::sync::Notify;
use tokio::sync::broadcast::error::RecvError;
use tracing::debug;

use crate::battery_indicator::BatteryIndicator;
use crate::dice_3d::Dice3D;
use crate::face_display::FaceDisplay;
use crate::face_display::RollHistory;

/// Interval for periodic battery level refresh.
const BATTERY_REFRESH_INTERVAL_SECS: u64 = 30;

/// Interval for battery level refresh while charging.
const BATTERY_CHARGING_REFRESH_INTERVAL_SECS: u64 = 1;

/// Delay before first reconnect attempt after a disconnect.
const RECONNECT_INITIAL_DELAY_SECS: u64 = 2;

/// Polling interval for draining UI updates on the GTK main thread (milliseconds).
const UI_POLL_INTERVAL_MS: u64 = 10;

/// UI update commands sent from the tokio event loop to the GTK main thread.
enum UiUpdate {
    Rolling,
    Stable { face: FaceValue, acceleration: Acceleration },
    TiltStable { face: FaceValue, acceleration: Acceleration },
    FakeStable { face: FaceValue, acceleration: Acceleration },
    MoveStable { face: FaceValue, acceleration: Acceleration },
    Charging { charging: bool },
    Disconnected,
    BatteryLevel(u8),
}

/// Bridges async dice events into the GTK main loop.
///
/// The event loop runs on a `tokio::spawn` task so it never blocks the GTK
/// main thread. UI updates are marshaled back via a `std::sync::mpsc` channel
/// that is polled on the GTK side with `glib::timeout_add_local`.
pub struct EventController {
    dice: Dice,
    manager: Arc<DiceManager>,
    face_display: FaceDisplay,
    battery_indicator: BatteryIndicator,
    dice_3d: Dice3D,
    roll_history: RollHistory,
}

impl EventController {
    /// Create a new event controller.
    pub fn new(dice: Dice, manager: Arc<DiceManager>, face_display: FaceDisplay, battery_indicator: BatteryIndicator, dice_3d: Dice3D, roll_history: RollHistory) -> Self {
        Self {
            dice,
            manager,
            face_display,
            battery_indicator,
            dice_3d,
            roll_history,
        }
    }

    /// Start listening for dice events and updating widgets.
    pub fn start(&self) {
        let (sender, receiver) = mpsc::channel::<UiUpdate>();

        // GTK main thread: poll the channel and apply UI updates.
        let face_display = self.face_display.clone();
        let dice_3d = self.dice_3d.clone();
        let roll_history = self.roll_history.clone();
        let battery_indicator = self.battery_indicator.clone();
        glib::timeout_add_local(Duration::from_millis(UI_POLL_INTERVAL_MS), move || {
            // Drain all pending updates in one batch.
            while let Ok(update) = receiver.try_recv() {
                match update {
                    UiUpdate::Rolling => {
                        face_display.set_rolling();
                        face_display.set_stability(StabilityDescriptor::Rolling);
                    }
                    UiUpdate::Stable { face, acceleration } => {
                        face_display.set_face(face);
                        face_display.set_stability(StabilityDescriptor::Stable);
                        roll_history.add_roll(face, StabilityDescriptor::Stable);
                        dice_3d.set_orientation(acceleration);
                    }
                    UiUpdate::TiltStable { face, acceleration } => {
                        face_display.set_face(face);
                        face_display.set_tilted(true);
                        face_display.set_stability(StabilityDescriptor::TiltStable);
                        roll_history.add_roll(face, StabilityDescriptor::TiltStable);
                        dice_3d.set_orientation(acceleration);
                    }
                    UiUpdate::FakeStable { face, acceleration } => {
                        face_display.set_face(face);
                        face_display.set_fake(true);
                        face_display.set_stability(StabilityDescriptor::FakeStable);
                        roll_history.add_roll(face, StabilityDescriptor::FakeStable);
                        dice_3d.set_orientation(acceleration);
                    }
                    UiUpdate::MoveStable { face, acceleration } => {
                        face_display.set_face(face);
                        face_display.set_stability(StabilityDescriptor::MoveStable);
                        roll_history.add_roll(face, StabilityDescriptor::MoveStable);
                        dice_3d.set_orientation(acceleration);
                    }
                    UiUpdate::Charging { charging } => {
                        battery_indicator.set_charging(charging);
                    }
                    UiUpdate::Disconnected => {
                        face_display.set_disconnected();
                        face_display.set_stability(StabilityDescriptor::Rolling);
                    }
                    UiUpdate::BatteryLevel(level) => {
                        battery_indicator.set_level(level);
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Tokio task: event loop — runs entirely off the GTK main thread.
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
                    Ok(DiceEvent::Charging { charging }) => {
                        charging_flag.store(charging, Ordering::Relaxed);
                        if charging {
                            charging_notify.notify_one();
                        }
                        let _ = event_sender.send(UiUpdate::Charging { charging });
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

        // Tokio task: periodic battery refresh — fully off the GTK main thread.
        // Polls more frequently while the dice is charging.
        let battery_dice = self.dice.clone();
        let battery_name = self.dice.name().to_string();
        let battery_sender = sender;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            match battery_dice.get_battery_level().await {
                Ok(level) => {
                    let _ = battery_sender.send(UiUpdate::BatteryLevel(level.get()));
                }
                Err(error) => debug!(device = %battery_name, error = %error, "initial battery fetch failed"),
            }

            loop {
                // Use dice.is_charging() as ground truth — the broadcast event
                // can be missed (lag), but the AtomicBool in DiceInner is always
                // updated by the notification task.
                let charging = battery_dice.is_charging();
                let prev = charging_flag_for_battery.swap(charging, Ordering::Relaxed);
                if charging != prev {
                    debug!(device = %battery_name, charging, prev, "charging state sync via battery task");
                    let _ = battery_sender.send(UiUpdate::Charging { charging });
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
                        let _ = battery_sender.send(UiUpdate::BatteryLevel(level.get()));
                    }
                    Err(error) => debug!(device = %battery_name, error = %error, charging, "battery refresh failed"),
                }
            }
        });
    }
}
