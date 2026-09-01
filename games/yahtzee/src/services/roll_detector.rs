use crate::models::dice_slot::DiceSlot;
use dice_rs::DiceEvent;
use dice_rs::FaceValue;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::broadcast;
use tracing::debug;

/// Timeout for waiting for all dice to become stable after a roll starts.
const ROLL_COMPLETION_TIMEOUT: Duration = Duration::from_secs(10);

/// The state of the roll detection for the dice set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollState {
    /// No roll in progress. All dice are stable.
    Idle,
    /// A roll has started (at least one dice reported RollStart).
    Rolling,
    /// All dice have reported stable face values.
    Complete,
    /// The roll timed out before all dice became stable.
    TimedOut,
}

/// Event emitted when a roll completes or times out.
#[derive(Debug, Clone, PartialEq)]
pub enum RollDetectorEvent {
    /// A roll has started on one or more dice.
    RollStarted,
    /// All dice are now stable. Contains the face values for each slot.
    RollComplete {
        /// Face values indexed by slot (0-4). `None` if a dice didn't report.
        faces: [Option<FaceValue>; 5],
    },
    /// The roll timed out. Some dice may not have reported stable values.
    RollTimedOut {
        /// Partial face values received before timeout.
        faces: [Option<FaceValue>; 5],
    },
    /// A single dice became stable.
    DiceStable {
        /// The slot that became stable.
        slot: DiceSlot,
        /// The face value reported.
        face: FaceValue,
    },
    /// A dice disconnected during a roll.
    DiceDisconnected {
        /// The slot that disconnected.
        slot: DiceSlot,
    },
}

/// Detects roll completion across all 5 dice.
///
/// Listens to `DiceEvent`s from each physical die and determines when
/// a roll is complete (all dice stable) or has timed out. Events are
/// emitted via a `tokio::sync::broadcast` channel.
#[derive(Clone)]
pub struct RollDetector {
    /// Current face values per slot (None if not yet reported for this roll).
    faces: Arc<Mutex<[Option<FaceValue>; 5]>>,
    /// Whether each slot has reported RollStart for the current roll.
    rolling: Arc<Mutex<[bool; 5]>>,
    /// Whether each slot has reported Stable for the current roll.
    stable: Arc<Mutex<[bool; 5]>>,
    /// Whether a roll is currently in progress.
    state: Arc<Mutex<RollState>>,
    /// Timestamp when the roll started.
    roll_start_time: Arc<Mutex<Option<Instant>>>,
    /// Broadcast channel for roll detector events.
    event_sender: broadcast::Sender<RollDetectorEvent>,
}

impl RollDetector {
    /// Create a new roll detector.
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(32);
        Self {
            faces: Arc::new(Mutex::new([None; 5])),
            rolling: Arc::new(Mutex::new([false; 5])),
            stable: Arc::new(Mutex::new([false; 5])),
            state: Arc::new(Mutex::new(RollState::Idle)),
            roll_start_time: Arc::new(Mutex::new(None)),
            event_sender,
        }
    }

    /// Subscribe to roll detector events.
    pub fn subscribe(&self) -> broadcast::Receiver<RollDetectorEvent> {
        self.event_sender.subscribe()
    }

    /// Get the current roll state.
    pub fn state(&self) -> RollState {
        self.state.lock().map(|s| *s).unwrap_or(RollState::Idle)
    }

    /// Reset the detector for a new roll.
    pub fn reset(&self) {
        if let Ok(mut faces) = self.faces.lock() {
            *faces = [None; 5];
        }
        if let Ok(mut rolling) = self.rolling.lock() {
            *rolling = [false; 5];
        }
        if let Ok(mut stable) = self.stable.lock() {
            *stable = [false; 5];
        }
        if let Ok(mut state) = self.state.lock() {
            *state = RollState::Idle;
        }
        if let Ok(mut start_time) = self.roll_start_time.lock() {
            *start_time = None;
        }
    }

    /// Handle a `DiceEvent` from a specific slot.
    ///
    /// This is the main entry point for feeding BLE events into the
    /// roll detector. The caller is responsible for mapping the dice
    /// name to a slot via `SlotMapping`.
    pub fn handle_event(&self, slot: DiceSlot, event: &DiceEvent) {
        match event {
            DiceEvent::RollStart => self.handle_roll_start(slot),
            DiceEvent::Stable { face, .. } => self.handle_stable(slot, *face),
            DiceEvent::TiltStable { face, .. } => self.handle_stable(slot, *face),
            DiceEvent::FakeStable { face, .. } => self.handle_stable(slot, *face),
            DiceEvent::MoveStable { face, .. } => self.handle_stable(slot, *face),
            DiceEvent::Disconnected => self.handle_disconnect(slot),
            _ => {}
        }
    }

    /// Handle a RollStart event from a slot.
    fn handle_roll_start(&self, slot: DiceSlot) {
        let slot_idx = slot.get() as usize;
        let should_emit = {
            let mut state = match self.state.lock() {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut rolling = match self.rolling.lock() {
                Ok(r) => r,
                Err(_) => return,
            };
            let mut stable = match self.stable.lock() {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut faces = match self.faces.lock() {
                Ok(f) => f,
                Err(_) => return,
            };

            rolling[slot_idx] = true;
            stable[slot_idx] = false;
            faces[slot_idx] = None;

            if *state == RollState::Idle {
                *state = RollState::Rolling;
                if let Ok(mut start_time) = self.roll_start_time.lock() {
                    *start_time = Some(Instant::now());
                }
                true
            } else {
                false
            }
        };

        if should_emit {
            let _ = self.event_sender.send(RollDetectorEvent::RollStarted);
            self.spawn_timeout_check();
        }
    }

    /// Handle a Stable event from a slot.
    fn handle_stable(&self, slot: DiceSlot, face: FaceValue) {
        let slot_idx = slot.get() as usize;
        let all_stable = {
            let state = match self.state.lock() {
                Ok(s) => s,
                Err(_) => return,
            };
            // Only process stable events during a roll
            if *state != RollState::Rolling {
                return;
            }

            let mut stable = match self.stable.lock() {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut faces = match self.faces.lock() {
                Ok(f) => f,
                Err(_) => return,
            };

            stable[slot_idx] = true;
            faces[slot_idx] = Some(face);

            // Check if all dice are stable
            stable.iter().all(|&s| s)
        };

        let _ = self.event_sender.send(RollDetectorEvent::DiceStable { slot, face });

        if all_stable {
            self.complete_roll();
        }
    }

    /// Handle a disconnection from a slot.
    fn handle_disconnect(&self, slot: DiceSlot) {
        let _ = self.event_sender.send(RollDetectorEvent::DiceDisconnected { slot });
    }

    /// Mark the roll as complete and emit the event.
    fn complete_roll(&self) {
        let faces = {
            let mut faces = match self.faces.lock() {
                Ok(f) => f,
                Err(_) => return,
            };
            let result = *faces;
            *faces = [None; 5];
            result
        };
        if let Ok(mut state) = self.state.lock() {
            *state = RollState::Complete;
        }
        if let Ok(mut rolling) = self.rolling.lock() {
            *rolling = [false; 5];
        }
        if let Ok(mut stable) = self.stable.lock() {
            *stable = [false; 5];
        }
        if let Ok(mut start_time) = self.roll_start_time.lock() {
            *start_time = None;
        }
        let _ = self.event_sender.send(RollDetectorEvent::RollComplete { faces });
    }

    /// Spawn a timeout check task for the current roll.
    fn spawn_timeout_check(&self) {
        let state = self.state.clone();
        let stable = self.stable.clone();
        let faces = self.faces.clone();
        let rolling = self.rolling.clone();
        let start_time = self.roll_start_time.clone();
        let sender = self.event_sender.clone();

        tokio::spawn(async move {
            tokio::time::sleep(ROLL_COMPLETION_TIMEOUT).await;

            let timed_out = {
                let s = match state.lock() {
                    Ok(s) => s,
                    Err(_) => return,
                };
                *s == RollState::Rolling
            };

            if !timed_out {
                return;
            }

            let partial_faces = {
                let mut f = match faces.lock() {
                    Ok(f) => f,
                    Err(_) => return,
                };
                let result = *f;
                *f = [None; 5];
                result
            };

            if let Ok(mut s) = state.lock() {
                *s = RollState::TimedOut;
            }
            if let Ok(mut r) = rolling.lock() {
                *r = [false; 5];
            }
            if let Ok(mut st) = stable.lock() {
                *st = [false; 5];
            }
            if let Ok(mut st) = start_time.lock() {
                *st = None;
            }

            debug!("roll timed out, partial faces: {:?}", partial_faces);
            let _ = sender.send(RollDetectorEvent::RollTimedOut { faces: partial_faces });
        });
    }

    /// Get the current face values (if any) for each slot.
    pub fn faces(&self) -> [Option<FaceValue>; 5] {
        self.faces.lock().map(|f| *f).unwrap_or([None; 5])
    }
}

impl Default for RollDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dice_rs::Acceleration;

    #[test]
    fn new_starts_idle() {
        let detector = RollDetector::new();
        assert_eq!(detector.state(), RollState::Idle);
    }

    #[test]
    fn reset_clears_state() {
        let detector = RollDetector::new();
        detector.reset();
        assert_eq!(detector.state(), RollState::Idle);
        assert_eq!(detector.faces(), [None; 5]);
    }

    #[tokio::test]
    async fn handle_roll_start_transitions_to_rolling() {
        let detector = RollDetector::new();
        let slot = DiceSlot::new(0).unwrap();
        detector.handle_event(slot, &DiceEvent::RollStart);
        assert_eq!(detector.state(), RollState::Rolling);
    }

    #[tokio::test]
    async fn handle_stable_without_roll_ignored() {
        let detector = RollDetector::new();
        let slot = DiceSlot::new(0).unwrap();
        let face = FaceValue::new(3).unwrap();
        detector.handle_event(
            slot,
            &DiceEvent::Stable {
                face,
                acceleration: Acceleration::default(),
            },
        );
        // Should stay idle since no roll was started
        assert_eq!(detector.state(), RollState::Idle);
    }

    #[tokio::test]
    async fn handle_all_stable_completes_roll() {
        let detector = RollDetector::new();
        let face = FaceValue::new(6).unwrap();
        let accel = Acceleration::default();

        // Start roll on all dice
        for slot in DiceSlot::all() {
            detector.handle_event(slot, &DiceEvent::RollStart);
        }
        assert_eq!(detector.state(), RollState::Rolling);

        // Report stable for all dice
        for slot in DiceSlot::all() {
            detector.handle_event(slot, &DiceEvent::Stable { face, acceleration: accel });
        }

        assert_eq!(detector.state(), RollState::Complete);
        let faces = detector.faces();
        assert_eq!(faces, [None; 5]); // faces cleared after completion
    }

    #[tokio::test]
    async fn handle_partial_stable_does_not_complete() {
        let detector = RollDetector::new();
        let face = FaceValue::new(4).unwrap();
        let accel = Acceleration::default();

        for slot in DiceSlot::all() {
            detector.handle_event(slot, &DiceEvent::RollStart);
        }

        // Only 3 dice stable
        for i in 0..3 {
            let slot = DiceSlot::new(i).unwrap();
            detector.handle_event(slot, &DiceEvent::Stable { face, acceleration: accel });
        }

        assert_eq!(detector.state(), RollState::Rolling);
    }

    #[test]
    fn handle_disconnect_emits_event() {
        let detector = RollDetector::new();
        let slot = DiceSlot::new(2).unwrap();
        let mut receiver = detector.subscribe();
        detector.handle_event(slot, &DiceEvent::Disconnected);
        let event = receiver.try_recv();
        assert!(event.is_ok());
    }
}
