use crate::models::dice_slot::DiceSlot;
use serde::Deserialize;
use serde::Serialize;

/// A mask indicating which dice in the set are held.
///
/// Held dice are not re-rolled on the next roll. The mask has one
/// boolean per dice slot (5 dice total).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldMask {
    /// One boolean per dice slot. `true` means the die is held.
    held: [bool; DiceSlot::COUNT],
}

impl HoldMask {
    /// Create a hold mask where no dice are held.
    pub fn none() -> Self {
        Self {
            held: [false; DiceSlot::COUNT],
        }
    }

    /// Create a hold mask where all dice are held.
    pub fn all() -> Self {
        Self { held: [true; DiceSlot::COUNT] }
    }

    /// Create a hold mask from an array of booleans.
    pub fn from_array(held: [bool; DiceSlot::COUNT]) -> Self {
        Self { held }
    }

    /// Get the held state for a specific dice slot.
    pub fn is_held(&self, slot: DiceSlot) -> bool {
        self.held[slot.get() as usize]
    }

    /// Toggle the held state for a dice slot.
    pub fn toggle(&mut self, slot: DiceSlot) {
        self.held[slot.get() as usize] = !self.held[slot.get() as usize];
    }

    /// Set the held state for a specific dice slot.
    pub fn set(&mut self, slot: DiceSlot, held: bool) {
        self.held[slot.get() as usize] = held;
    }

    /// Returns the number of held dice.
    pub fn held_count(&self) -> usize {
        self.held.iter().filter(|&&h| h).count()
    }

    /// Returns the number of dice that will be re-rolled.
    pub fn reroll_count(&self) -> usize {
        DiceSlot::COUNT - self.held_count()
    }

    /// Returns the underlying array of held states.
    pub fn as_array(&self) -> [bool; DiceSlot::COUNT] {
        self.held
    }
}

impl Default for HoldMask {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_holds_nothing() {
        let mask = HoldMask::none();
        for slot in DiceSlot::all() {
            assert!(!mask.is_held(slot));
        }
        assert_eq!(mask.held_count(), 0);
        assert_eq!(mask.reroll_count(), 5);
    }

    #[test]
    fn all_holds_everything() {
        let mask = HoldMask::all();
        for slot in DiceSlot::all() {
            assert!(mask.is_held(slot));
        }
        assert_eq!(mask.held_count(), 5);
        assert_eq!(mask.reroll_count(), 0);
    }

    #[test]
    fn toggle() {
        let mut mask = HoldMask::none();
        let slot = DiceSlot::new(2).unwrap();
        mask.toggle(slot);
        assert!(mask.is_held(slot));
        mask.toggle(slot);
        assert!(!mask.is_held(slot));
    }

    #[test]
    fn set() {
        let mut mask = HoldMask::none();
        let slot = DiceSlot::new(1).unwrap();
        mask.set(slot, true);
        assert!(mask.is_held(slot));
        assert_eq!(mask.held_count(), 1);
    }

    #[test]
    fn from_array() {
        let mask = HoldMask::from_array([true, false, true, false, true]);
        assert_eq!(mask.held_count(), 3);
        assert_eq!(mask.reroll_count(), 2);
    }
}
