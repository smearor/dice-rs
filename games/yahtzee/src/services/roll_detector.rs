use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use dice_rs::DiceColor;
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

/// Grace period after all rolling dice are stable before completing the roll.
///
/// BLE notifications from different dice arrive at slightly different times.
/// This delay gives late-reporting dice a chance to send their RollStart and
/// Stable events before the roll is finalized.
const ROLL_GRACE_PERIOD: Duration = Duration::from_millis(300);

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
    /// Monotonically increasing roll ID, used to ignore stale timeout tasks.
    roll_id: Arc<Mutex<u64>>,
    /// Whether the roll is waiting for held dice to be marked stable.
    ///
    /// Set to `true` when a roll starts during Holding phase (physical
    /// pickup). The roll cannot complete until `mark_held_stable` clears
    /// this flag. This prevents a premature RollComplete when a picked-up
    /// dice reports Stable before the auto-hold timer fires.
    pending_holds: Arc<Mutex<bool>>,
    /// Physical dice color per slot, for diagnostic logging.
    slot_colors: Arc<Mutex<[Option<DiceColor>; 5]>>,
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
            roll_id: Arc::new(Mutex::new(0)),
            pending_holds: Arc::new(Mutex::new(false)),
            slot_colors: Arc::new(Mutex::new([None; 5])),
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

    /// Set whether the roll is waiting for held dice to be marked stable.
    ///
    /// Call `set_pending_holds(true)` when a roll starts from a physical
    /// pickup during Holding phase. The roll will not complete until
    /// `mark_held_stable` clears the flag.
    pub fn set_pending_holds(&self, pending: bool) {
        if let Ok(mut p) = self.pending_holds.lock() {
            *p = pending;
        }
    }

    /// Set the physical dice color for a slot, used in diagnostic logs.
    pub fn set_slot_color(&self, slot: DiceSlot, color: DiceColor) {
        if let Ok(mut colors) = self.slot_colors.lock() {
            colors[slot.get() as usize] = Some(color);
        }
    }

    /// Get the color label for a slot for use in log messages.
    fn color_label(&self, slot_idx: usize) -> String {
        self.slot_colors
            .lock()
            .ok()
            .and_then(|colors| colors[slot_idx])
            .map(|c| c.to_string())
            .unwrap_or_else(|| "?".to_string())
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
        if let Ok(mut id) = self.roll_id.lock() {
            *id += 1;
        }
        if let Ok(mut p) = self.pending_holds.lock() {
            *p = false;
        }
    }

    /// Reset the detector for a new roll, preserving held dice.
    ///
    /// Held slots are marked as already stable so the detector only
    /// waits for non-held dice to report their face values.
    pub fn reset_with_holds(&self, holds: HoldMask, current_faces: [Option<FaceValue>; 5]) {
        if let Ok(mut faces) = self.faces.lock() {
            for slot in DiceSlot::all() {
                let idx = slot.get() as usize;
                if holds.is_held(slot) {
                    faces[idx] = current_faces[idx];
                } else {
                    faces[idx] = None;
                }
            }
        }
        if let Ok(mut rolling) = self.rolling.lock() {
            *rolling = [false; 5];
        }
        if let Ok(mut stable) = self.stable.lock() {
            for slot in DiceSlot::all() {
                let idx = slot.get() as usize;
                stable[idx] = holds.is_held(slot);
            }
        }
        if let Ok(mut state) = self.state.lock() {
            *state = RollState::Idle;
        }
        if let Ok(mut start_time) = self.roll_start_time.lock() {
            *start_time = None;
        }
        if let Ok(mut id) = self.roll_id.lock() {
            *id += 1;
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
        let color = self.color_label(slot_idx);
        debug!(slot = slot_idx, color = %color, "RollStart received");
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
                // Increment roll ID so stale timeout tasks from previous rolls are ignored
                let current_roll_id = if let Ok(mut id) = self.roll_id.lock() {
                    *id += 1;
                    *id
                } else {
                    0
                };
                Some(current_roll_id)
            } else {
                None
            }
        };

        if let Some(roll_id) = should_emit {
            debug!(roll_id, "emitting RollStarted, spawning timeout");
            let _ = self.event_sender.send(RollDetectorEvent::RollStarted);
            self.spawn_timeout_check(roll_id);
        } else {
            debug!(slot = slot_idx, color = %self.color_label(slot_idx), "RollStart ignored (already rolling)");
        }
    }

    /// Handle a Stable event from a slot.
    fn handle_stable(&self, slot: DiceSlot, face: FaceValue) {
        let slot_idx = slot.get() as usize;
        let color = self.color_label(slot_idx);
        debug!(slot = slot_idx, color = %color, face = face.get(), "Stable received");
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
            // Update face value on duplicate Stable events.
            //
            // GoDice firmware may send multiple stable notifications as the die
            // settles (e.g. MoveStable then Stable). The last notification before
            // the grace period expires has the most accurate face value, so we
            // always update the stored face. We do NOT re-trigger the all-stable
            // check — the slot is already marked stable.
            if stable[slot_idx] {
                debug!(slot = slot_idx, color = %color, face = face.get(), "Stable face updated (already stable)");
                if let Ok(mut faces) = self.faces.lock() {
                    faces[slot_idx] = Some(face);
                }
                return;
            }
            let mut faces = match self.faces.lock() {
                Ok(f) => f,
                Err(_) => return,
            };
            let rolling = match self.rolling.lock() {
                Ok(r) => r,
                Err(_) => return,
            };
            let pending = match self.pending_holds.lock() {
                Ok(p) => *p,
                Err(_) => return,
            };

            stable[slot_idx] = true;
            faces[slot_idx] = Some(face);

            // Don't complete if waiting for held dice to be marked stable.
            if pending {
                debug!(slot = slot_idx, color = %color, "completion blocked by pending_holds");
                false
            } else {
                // Check if all dice that reported RollStart are now stable.
                // Dice that never sent RollStart (e.g. BLE notification issues)
                // do not block roll completion.
                rolling.iter().zip(stable.iter()).all(|(&r, &s)| !r || s)
            }
        };

        let _ = self.event_sender.send(RollDetectorEvent::DiceStable { slot, face });

        if all_stable {
            debug!(slot = slot_idx, color = %color, "all rolling dice stable, starting grace period");
            self.spawn_grace_completion();
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
        let colors = self.slot_colors.lock().map(|c| *c).unwrap_or([None; 5]);
        debug!(
            faces = ?faces.iter().map(|f| f.map(|v| v.get())).collect::<Vec<_>>(),
            colors = ?colors.iter().map(|c| c.map(|d| d.to_string())).collect::<Vec<_>>(),
            "complete_roll"
        );
        if let Ok(mut state) = self.state.lock() {
            *state = RollState::Idle;
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
        if let Ok(mut id) = self.roll_id.lock() {
            *id += 1;
        }
        let _ = self.event_sender.send(RollDetectorEvent::RollComplete { faces });
    }

    /// Mark held dice as already stable with their current face values.
    ///
    /// This is used after auto-hold-on-pickup: the RollDetector is already
    /// tracking the roll (state == Rolling), and we need to tell it that
    /// the held dice are already "stable" so it only waits for the
    /// picked-up dice.
    ///
    /// If all dice are now stable after marking, `complete_roll` is triggered.
    pub fn mark_held_stable(&self, holds: HoldMask, current_faces: [Option<FaceValue>; 5]) {
        let all_stable = {
            let state = match self.state.lock() {
                Ok(s) => s,
                Err(_) => return,
            };
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
            let rolling = match self.rolling.lock() {
                Ok(r) => r,
                Err(_) => return,
            };
            // Clear pending_holds — held dice are now being marked stable
            if let Ok(mut p) = self.pending_holds.lock() {
                debug!(pending = *p, "clearing pending_holds in mark_held_stable");
                *p = false;
            }
            for slot in DiceSlot::all() {
                let idx = slot.get() as usize;
                if holds.is_held(slot) {
                    if stable[idx] {
                        // Already received a Stable event for this held die
                        // (e.g. MoveStable from a jostled die). Keep the
                        // RollDetector's face value — it reflects the die's
                        // actual current face, not the stale controller value.
                        debug!(slot = idx, color = %self.color_label(idx), face = ?faces[idx].map(|f| f.get()), "mark_held_stable skipped (already stable)");
                        continue;
                    }
                    let color = self.color_label(idx);
                    debug!(slot = idx, color = %color, face = ?current_faces[idx].map(|f| f.get()), "mark_held_stable");
                    stable[idx] = true;
                    faces[idx] = current_faces[idx];
                }
            }
            rolling.iter().zip(stable.iter()).all(|(&r, &s)| !r || s)
        };
        if all_stable {
            debug!("all dice stable after mark_held_stable, starting grace period");
            self.spawn_grace_completion();
        }
    }

    /// Spawn a grace period task that completes the roll after a short delay.
    ///
    /// When all rolling dice are stable, we wait `ROLL_GRACE_PERIOD` before
    /// completing. This gives late-reporting dice (whose BLE RollStart/Stable
    /// notifications are delayed) a chance to be included. If a new RollStart
    /// arrives during the grace period, the all-stable check will fail and
    /// the task exits without completing.
    fn spawn_grace_completion(&self) {
        let detector = self.clone();
        tokio::spawn(async move {
            tokio::time::sleep(ROLL_GRACE_PERIOD).await;

            let should_complete = {
                let state = match detector.state.lock() {
                    Ok(s) => s,
                    Err(_) => return,
                };
                if *state != RollState::Rolling {
                    false
                } else {
                    let stable = match detector.stable.lock() {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    let rolling = match detector.rolling.lock() {
                        Ok(r) => r,
                        Err(_) => return,
                    };
                    rolling.iter().zip(stable.iter()).all(|(&r, &s)| !r || s)
                }
            };

            if should_complete {
                debug!("grace period expired, completing roll");
                detector.complete_roll();
            }
        });
    }

    /// Spawn a timeout check task for the current roll.
    ///
    /// The `roll_id` identifies which roll this timeout belongs to.
    /// If a new roll starts before this timeout fires, the roll_id will
    /// have changed and the timeout is ignored.
    fn spawn_timeout_check(&self, expected_roll_id: u64) {
        let state = self.state.clone();
        let stable = self.stable.clone();
        let faces = self.faces.clone();
        let rolling = self.rolling.clone();
        let start_time = self.roll_start_time.clone();
        let roll_id = self.roll_id.clone();
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

            // Ignore stale timeout from a previous roll
            let current_roll_id = match roll_id.lock() {
                Ok(id) => *id,
                Err(_) => return,
            };
            if current_roll_id != expected_roll_id {
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

        // Roll completes after grace period
        tokio::time::sleep(Duration::from_millis(400)).await;
        assert_eq!(detector.state(), RollState::Idle);
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
