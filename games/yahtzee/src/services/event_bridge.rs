use crate::models::dice_slot::DiceSlot;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::services::led_effect::LedEffect;
use crate::services::roll_detector::RollDetector;
use crate::services::roll_detector::RollDetectorEvent;
use crate::services::slot_mapping::SlotMapping;
use dice_rs::Dice;
use dice_rs::DiceEvent;
use dice_rs::FaceValue;
use tokio::sync::broadcast;
use tracing::debug;

/// High-level game events derived from BLE dice events.
///
/// These events represent the game-relevant interpretation of raw
/// `DiceEvent`s. The UI layer consumes these instead of raw BLE events.
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    /// A roll has started on one or more dice.
    RollStarted,
    /// All dice are now stable. Contains the face values for each slot.
    RollComplete {
        /// Face values indexed by slot (0-4). `None` if a dice didn't report.
        faces: [Option<FaceValue>; 5],
    },
    /// The roll timed out before all dice became stable.
    RollTimedOut {
        /// Partial face values received before timeout.
        faces: [Option<FaceValue>; 5],
    },
    /// A single dice became stable with a face value.
    DiceStable {
        /// The slot that became stable.
        slot: DiceSlot,
        /// The face value reported.
        face: FaceValue,
    },
    /// A dice disconnected from its slot.
    DiceDisconnected {
        /// The slot that disconnected.
        slot: DiceSlot,
        /// The device name (for reconnection).
        name: String,
    },
    /// A dice reconnected to its slot.
    DiceReconnected {
        /// The slot that reconnected.
        slot: DiceSlot,
    },
    /// An LED effect should be applied (e.g. celebration).
    ApplyLedEffect {
        /// The effect to apply.
        effect: LedEffect,
    },
    /// The active player changed. LEDs should be updated.
    ActivePlayerChanged {
        /// The new active player's index.
        player_index: PlayerIndex,
        /// The new active player's color.
        color: PlayerColor,
    },
}

/// Bridges raw BLE `DiceEvent`s into high-level `GameEvent`s.
///
/// The event bridge runs as a background task that subscribes to each
/// connected dice's event stream, maps events through the `SlotMapping`
/// and `RollDetector`, and emits `GameEvent`s on a broadcast channel.
#[derive(Clone)]
pub struct EventBridge {
    /// Slot mapping for dice name → slot lookup.
    mapping: SlotMapping,
    /// Roll detector for tracking roll completion.
    roll_detector: RollDetector,
    /// Broadcast channel for game events.
    event_sender: broadcast::Sender<GameEvent>,
}

impl EventBridge {
    /// Create a new event bridge.
    pub fn new(mapping: SlotMapping, roll_detector: RollDetector) -> Self {
        let (event_sender, _) = broadcast::channel(64);
        Self {
            mapping,
            roll_detector,
            event_sender,
        }
    }

    /// Subscribe to game events.
    pub fn subscribe(&self) -> broadcast::Receiver<GameEvent> {
        self.event_sender.subscribe()
    }

    /// Get a clone of the roll detector.
    pub fn roll_detector(&self) -> RollDetector {
        self.roll_detector.clone()
    }

    /// Start listening for events from a specific dice.
    ///
    /// Spawns a tokio task that subscribes to the dice's event stream
    /// and forwards events through the bridge. The task exits when the
    /// dice disconnects permanently.
    pub fn start_listening(&self, slot: DiceSlot, dice: Dice) {
        let mapping = self.mapping.clone();
        let roll_detector = self.roll_detector.clone();
        let sender = self.event_sender.clone();
        let slot_idx = slot.get();
        let dice_name = dice.name().to_string();

        tokio::spawn(async move {
            let mut receiver = dice.subscribe();

            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        Self::handle_dice_event(slot, &event, &roll_detector, &sender, &mapping, &dice_name);
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        debug!(slot = slot_idx, skipped, "event receiver lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        debug!(slot = slot_idx, "event channel closed, stopping listener");
                        break;
                    }
                }
            }
        });
    }

    /// Process a single dice event.
    fn handle_dice_event(
        slot: DiceSlot,
        event: &DiceEvent,
        roll_detector: &RollDetector,
        sender: &broadcast::Sender<GameEvent>,
        mapping: &SlotMapping,
        dice_name: &str,
    ) {
        // Feed into roll detector
        roll_detector.handle_event(slot, event);

        // Handle disconnection specially
        if let DiceEvent::Disconnected = event {
            let _ = sender.send(GameEvent::DiceDisconnected {
                slot,
                name: dice_name.to_string(),
            });
            // Remove from mapping
            mapping.remove(dice_name);
        }
    }

    /// Start listening for roll detector events and forwarding them
    /// as game events.
    ///
    /// This should be called once after the event bridge is created.
    /// It spawns a tokio task that listens to the roll detector's
    /// broadcast channel and re-emits events as `GameEvent`s.
    pub fn start_roll_detector_forwarding(&self) {
        let roll_detector = self.roll_detector.clone();
        let sender = self.event_sender.clone();

        tokio::spawn(async move {
            let mut receiver = roll_detector.subscribe();
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        let game_event = match event {
                            RollDetectorEvent::RollStarted => Some(GameEvent::RollStarted),
                            RollDetectorEvent::RollComplete { faces } => Some(GameEvent::RollComplete { faces }),
                            RollDetectorEvent::RollTimedOut { faces } => Some(GameEvent::RollTimedOut { faces }),
                            RollDetectorEvent::DiceStable { slot, face } => Some(GameEvent::DiceStable { slot, face }),
                            RollDetectorEvent::DiceDisconnected { slot: _ } => {
                                // The disconnect is already handled in handle_dice_event
                                // via the DiceEvent::Disconnected path. Skip duplicate.
                                None
                            }
                        };
                        if let Some(game_event) = game_event {
                            let _ = sender.send(game_event);
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        debug!(skipped, "roll detector receiver lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        debug!("roll detector channel closed");
                        break;
                    }
                }
            }
        });
    }

    /// Emit an active player change event.
    pub fn emit_active_player_changed(&self, player_index: PlayerIndex, color: PlayerColor) {
        let _ = self.event_sender.send(GameEvent::ActivePlayerChanged { player_index, color });
    }

    /// Emit an LED effect event.
    pub fn emit_led_effect(&self, effect: LedEffect) {
        let _ = self.event_sender.send(GameEvent::ApplyLedEffect { effect });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_bridge() {
        let mapping = SlotMapping::new();
        let detector = RollDetector::new();
        let bridge = EventBridge::new(mapping, detector);
        let _receiver = bridge.subscribe();
    }

    #[test]
    fn emit_active_player_changed_sends_event() {
        let mapping = SlotMapping::new();
        let detector = RollDetector::new();
        let bridge = EventBridge::new(mapping, detector);
        let mut receiver = bridge.subscribe();

        bridge.emit_active_player_changed(PlayerIndex::new(0), PlayerColor::RED);

        let event = receiver.try_recv();
        assert!(event.is_ok());
        match event.unwrap() {
            GameEvent::ActivePlayerChanged { player_index, color } => {
                assert_eq!(player_index, PlayerIndex::new(0));
                assert_eq!(color, PlayerColor::RED);
            }
            _ => panic!("expected ActivePlayerChanged event"),
        }
    }

    #[test]
    fn emit_led_effect_sends_event() {
        let mapping = SlotMapping::new();
        let detector = RollDetector::new();
        let bridge = EventBridge::new(mapping, detector);
        let mut receiver = bridge.subscribe();

        bridge.emit_led_effect(LedEffect::yahtzee());

        let event = receiver.try_recv();
        assert!(event.is_ok());
        match event.unwrap() {
            GameEvent::ApplyLedEffect { effect } => {
                assert_eq!(effect, LedEffect::yahtzee());
            }
            _ => panic!("expected ApplyLedEffect event"),
        }
    }
}
