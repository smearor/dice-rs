use crate::models::dice_slot::DiceSlot;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::services::game_event::GameEvent;
use crate::services::led_effect::LedEffect;
use crate::services::roll_detector::RollDetector;
use crate::services::roll_detector::RollDetectorEvent;
use crate::services::slot_mapping::SlotMapping;
use dice_rs::Dice;
use dice_rs::DiceColor;
use dice_rs::DiceEvent;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use tokio::sync::broadcast;
use tracing::debug;

/// Bridges raw BLE `DiceEvent`s into high-level `GameEvent`s.
///
/// The event bridge runs as a background task that subscribes to each
/// connected dice's event stream, maps events through the `SlotMapping`
/// and `RollDetector`, and emits `GameEvent`s on a broadcast channel.
#[derive(Clone)]
pub struct EventBridge {
    /// Roll detector for tracking roll completion.
    roll_detector: RollDetector,
    /// Broadcast channel for game events.
    event_sender: broadcast::Sender<GameEvent>,
    /// Which slots currently have an active listener task.
    active_slots: Arc<Mutex<[bool; 5]>>,
    /// Whether roll detector forwarding has already been started.
    forwarding_started: Arc<AtomicBool>,
}

impl EventBridge {
    /// Create a new event bridge.
    pub fn new(_mapping: SlotMapping, roll_detector: RollDetector) -> Self {
        let (event_sender, _) = broadcast::channel(64);
        Self {
            roll_detector,
            event_sender,
            active_slots: Arc::new(Mutex::new([false; 5])),
            forwarding_started: Arc::new(AtomicBool::new(false)),
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

    /// Reset the active state for a slot, allowing a new listener to start.
    ///
    /// This is used when a dice is swapped: the old listener task may still
    /// be alive (holding the old Dice's broadcast channel open), so
    /// `active_slots` stays `true` and `start_listening` would skip the
    /// new dice. Calling this before `start_listening` ensures the new
    /// dice gets a listener.
    pub fn reset_active_slot(&self, slot: DiceSlot) {
        if let Ok(mut active) = self.active_slots.lock() {
            active[slot.get() as usize] = false;
        }
    }

    /// Start listening for events from a specific dice.
    ///
    /// Spawns a tokio task that subscribes to the dice's event stream
    /// and forwards events through the bridge. The task exits when the
    /// dice disconnects permanently.
    pub fn start_listening(&self, slot: DiceSlot, dice: Dice) {
        // Prevent duplicate listeners for the same slot
        {
            let mut active = match self.active_slots.lock() {
                Ok(a) => a,
                Err(_) => return,
            };
            let idx = slot.get() as usize;
            if active[idx] {
                debug!(slot = slot.get(), "start_listening ignored (already active)");
                return;
            }
            active[idx] = true;
        }
        let active_slots = self.active_slots.clone();
        let roll_detector = self.roll_detector.clone();
        let sender = self.event_sender.clone();
        let slot_idx = slot.get();
        let dice_name = dice.name().to_string();

        // Parse color from dice name and register it in the roll detector for logging.
        let color = parse_color_from_name(&dice_name);
        if let Some(c) = color {
            roll_detector.set_slot_color(slot, c);
        }
        debug!(slot = slot_idx, color = ?color, name = %dice_name, "start_listening");

        tokio::spawn(async move {
            let mut receiver = dice.subscribe();

            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        Self::handle_dice_event(slot, &event, &roll_detector, &sender, &dice_name);
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
            // Mark slot as no longer active when the task exits
            if let Ok(mut active) = active_slots.lock() {
                active[slot_idx as usize] = false;
            }
        });
    }

    /// Process a single dice event.
    fn handle_dice_event(slot: DiceSlot, event: &DiceEvent, roll_detector: &RollDetector, sender: &broadcast::Sender<GameEvent>, dice_name: &str) {
        let color = parse_color_from_name(dice_name);
        let slot_idx = slot.get();
        debug!(slot = slot_idx, color = ?color, ?event, "handle_dice_event");
        // Emit DicePickedUp for RollStart events so the UI can auto-hold other dice
        if let DiceEvent::RollStart = event {
            let _ = sender.send(GameEvent::DicePickedUp { slot });
        }

        // Emit DiceStable directly for all stable-type events, so the UI can
        // display face values even outside of a roll (e.g. on the setup screen).
        let stable_face = match event {
            DiceEvent::Stable { face, .. } => Some(*face),
            DiceEvent::TiltStable { face, .. } => Some(*face),
            DiceEvent::FakeStable { face, .. } => Some(*face),
            DiceEvent::MoveStable { face, .. } => Some(*face),
            _ => None,
        };
        if let Some(face) = stable_face {
            debug!(slot = slot_idx, color = ?color, face = face.get(), "emitting DiceStable");
            let _ = sender.send(GameEvent::DiceStable { slot, face });
        }

        // Feed into roll detector
        roll_detector.handle_event(slot, event);

        // Handle disconnection specially
        if let DiceEvent::Disconnected = event {
            debug!(slot = slot_idx, color = ?color, "dice disconnected");
            let _ = sender.send(GameEvent::DiceDisconnected {
                slot,
                name: dice_name.to_string(),
            });
            // Note: dice is NOT removed from mapping here, so the Dice
            // handle remains available for reconnection attempts.
        }
    }

    /// Start listening for roll detector events and forwarding them
    /// as game events.
    ///
    /// This should be called once after the event bridge is created.
    /// It spawns a tokio task that listens to the roll detector's
    /// broadcast channel and re-emits events as `GameEvent`s.
    pub fn start_roll_detector_forwarding(&self) {
        // Prevent duplicate forwarding tasks
        if self.forwarding_started.swap(true, Ordering::SeqCst) {
            debug!("start_roll_detector_forwarding ignored (already started)");
            return;
        }
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
                            RollDetectorEvent::DiceStable { slot: _, face: _ } => {
                                // Already emitted directly in handle_dice_event
                                None
                            }
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

/// Parse the dice color from a GoDice device name (e.g. "GoDice_320FF4_G_v04").
///
/// Returns `None` if the color code cannot be parsed.
fn parse_color_from_name(name: &str) -> Option<DiceColor> {
    let parts: Vec<&str> = name.split('_').collect();
    let code = parts.get(parts.len().saturating_sub(2))?;
    let ch = code.chars().next()?;
    DiceColor::try_from(ch).ok()
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

        bridge.emit_led_effect(LedEffect::yatzy());

        let event = receiver.try_recv();
        assert!(event.is_ok());
        match event.unwrap() {
            GameEvent::ApplyLedEffect { effect } => {
                assert_eq!(effect, LedEffect::yatzy());
            }
            _ => panic!("expected ApplyLedEffect event"),
        }
    }
}
