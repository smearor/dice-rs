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
use tokio::sync::broadcast::error::RecvError;
use tracing::debug;

use crate::battery_indicator::BatteryIndicator;
use crate::dice_3d::Dice3D;
use crate::face_display::FaceDisplay;
use crate::face_display::RollHistory;

/// Interval for periodic battery level refresh.
const BATTERY_REFRESH_INTERVAL_SECS: u64 = 30;

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
        let manager = self.manager.clone();
        let event_sender = sender.clone();
        tokio::spawn(async move {
            let mut receiver = dice.subscribe();
            loop {
                match receiver.recv().await {
                    Ok(DiceEvent::RollStart) => {
                        let _ = event_sender.send(UiUpdate::Rolling);
                    }
                    Ok(DiceEvent::Stable { face, acceleration }) => {
                        let _ = event_sender.send(UiUpdate::Stable { face, acceleration });
                    }
                    Ok(DiceEvent::TiltStable { face, acceleration }) => {
                        let _ = event_sender.send(UiUpdate::TiltStable { face, acceleration });
                    }
                    Ok(DiceEvent::FakeStable { face, acceleration }) => {
                        let _ = event_sender.send(UiUpdate::FakeStable { face, acceleration });
                    }
                    Ok(DiceEvent::MoveStable { face, acceleration }) => {
                        let _ = event_sender.send(UiUpdate::MoveStable { face, acceleration });
                    }
                    Ok(DiceEvent::Disconnected) => {
                        let _ = event_sender.send(UiUpdate::Disconnected);

                        // Auto-reconnect on the tokio threadpool.
                        tokio::time::sleep(Duration::from_secs(RECONNECT_INITIAL_DELAY_SECS)).await;
                        match manager.reconnect(&dice).await {
                            Ok(()) => {
                                debug!("reconnected successfully, resuming event loop");
                                receiver = dice.subscribe();
                            }
                            Err(error) => {
                                debug!(error = %error, "reconnect failed permanently");
                                break;
                            }
                        }
                    }
                    Err(RecvError::Lagged(skipped)) => {
                        debug!(skipped, "event receiver lagged, continuing");
                    }
                    Err(RecvError::Closed) => {
                        debug!("event channel closed, stopping event controller");
                        break;
                    }
                }
            }
        });

        // Tokio task: periodic battery refresh — fully off the GTK main thread.
        let battery_dice = self.dice.clone();
        let battery_sender = sender;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            match battery_dice.get_battery_level().await {
                Ok(level) => {
                    let _ = battery_sender.send(UiUpdate::BatteryLevel(level.get()));
                }
                Err(error) => debug!(error = %error, "initial battery fetch failed"),
            }

            loop {
                tokio::time::sleep(Duration::from_secs(BATTERY_REFRESH_INTERVAL_SECS)).await;
                match battery_dice.is_connected().await {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => {
                        debug!(error = %error, "battery refresh: is_connected failed");
                        continue;
                    }
                }
                match battery_dice.get_battery_level().await {
                    Ok(level) => {
                        let _ = battery_sender.send(UiUpdate::BatteryLevel(level.get()));
                    }
                    Err(error) => debug!(error = %error, "battery refresh failed"),
                }
            }
        });
    }
}
