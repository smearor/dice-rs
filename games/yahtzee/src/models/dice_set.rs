use crate::error::Result;
use crate::error::YahtzeeError;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use dice_rs::FaceValue;
use serde::Deserialize;
use serde::Serialize;

/// The 5 dice in a Kniffel game with their current face values and hold state.
///
/// `DiceSet` is the central data structure for representing the current
/// roll state. It tracks which face value each die shows and which dice
/// are held (not re-rolled on the next roll).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiceSet {
    /// The face values of the 5 dice, indexed by slot.
    faces: [FaceValue; DiceSlot::COUNT],
    /// Which dice are currently held.
    holds: HoldMask,
}

impl DiceSet {
    /// Create a new dice set with all dice showing 1 and none held.
    pub fn new() -> Self {
        Self {
            faces: [FaceValue::ONE; DiceSlot::COUNT],
            holds: HoldMask::none(),
        }
    }

    /// Create a dice set from face values. All dice start unheld.
    pub fn from_faces(faces: [FaceValue; DiceSlot::COUNT]) -> Self {
        Self {
            faces,
            holds: HoldMask::none(),
        }
    }

    /// Get the face value at a specific slot.
    pub fn face(&self, slot: DiceSlot) -> FaceValue {
        self.faces[slot.get() as usize]
    }

    /// Set the face value at a specific slot.
    pub fn set_face(&mut self, slot: DiceSlot, face: FaceValue) {
        self.faces[slot.get() as usize] = face;
    }

    /// Get all face values as an array.
    pub fn faces(&self) -> [FaceValue; DiceSlot::COUNT] {
        self.faces
    }

    /// Get the face values as raw `u8` values (1-6).
    pub fn values(&self) -> [u8; DiceSlot::COUNT] {
        [
            self.faces[0].get(),
            self.faces[1].get(),
            self.faces[2].get(),
            self.faces[3].get(),
            self.faces[4].get(),
        ]
    }

    /// Get the sorted face values as raw `u8` values (ascending).
    pub fn sorted_values(&self) -> [u8; DiceSlot::COUNT] {
        let mut values = self.values();
        values.sort_unstable();
        values
    }

    /// Get the hold mask.
    pub fn holds(&self) -> HoldMask {
        self.holds
    }

    /// Set the hold mask.
    pub fn set_holds(&mut self, holds: HoldMask) {
        self.holds = holds;
    }

    /// Toggle the hold state of a die.
    pub fn toggle_hold(&mut self, slot: DiceSlot) {
        self.holds.toggle(slot);
    }

    /// Returns true if the die at the given slot is held.
    pub fn is_held(&self, slot: DiceSlot) -> bool {
        self.holds.is_held(slot)
    }

    /// Count occurrences of each face value.
    ///
    /// Returns an array of length 7 where index `i` contains the count
    /// of dice showing face value `i`. Index 0 is always 0 (unused).
    pub fn counts(&self) -> [u8; 7] {
        let mut counts = [0u8; 7];
        for &value in &self.values() {
            counts[value as usize] += 1;
        }
        counts
    }

    /// Sum of all face values.
    pub fn sum(&self) -> u32 {
        self.values().iter().map(|&v| v as u32).sum()
    }

    /// Update face values from a new roll. All dice are updated —
    /// held dice get their current face from `mark_held_stable`, so
    /// updating them is a no-op in the normal case. If a held die
    /// reported a new face (e.g. MoveStable from being jostled),
    /// we use the new value.
    pub fn apply_roll(&mut self, new_faces: [FaceValue; DiceSlot::COUNT]) {
        for slot in DiceSlot::all() {
            self.faces[slot.get() as usize] = new_faces[slot.get() as usize];
        }
    }

    /// Reset all holds to none.
    pub fn clear_holds(&mut self) {
        self.holds = HoldMask::none();
    }

    /// Reset the dice set to initial state (all 1s, no holds).
    pub fn reset(&mut self) {
        self.faces = [FaceValue::ONE; DiceSlot::COUNT];
        self.holds = HoldMask::none();
    }

    /// Create a dice set from raw u8 values. Returns an error if any
    /// value is 0 (invalid face value).
    pub fn from_values(values: [u8; DiceSlot::COUNT]) -> Result<Self> {
        let mut faces = [FaceValue::ONE; DiceSlot::COUNT];
        for (i, &v) in values.iter().enumerate() {
            faces[i] = FaceValue::new(v).map_err(|_| YahtzeeError::InvalidFaceValue(v))?;
        }
        Ok(Self::from_faces(faces))
    }
}

impl Default for DiceSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_defaults_to_ones() {
        let ds = DiceSet::new();
        for slot in DiceSlot::all() {
            assert_eq!(ds.face(slot).get(), 1);
        }
        assert!(!ds.is_held(DiceSlot::new(0).unwrap()));
    }

    #[test]
    fn from_values_valid() {
        let ds = DiceSet::from_values([1, 2, 3, 4, 5]).unwrap();
        assert_eq!(ds.values(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn from_values_zero_fails() {
        assert!(DiceSet::from_values([0, 2, 3, 4, 5]).is_err());
    }

    #[test]
    fn counts() {
        let ds = DiceSet::from_values([3, 3, 3, 5, 5]).unwrap();
        let counts = ds.counts();
        assert_eq!(counts[3], 3);
        assert_eq!(counts[5], 2);
        assert_eq!(counts[1], 0);
    }

    #[test]
    fn sum() {
        let ds = DiceSet::from_values([1, 2, 3, 4, 5]).unwrap();
        assert_eq!(ds.sum(), 15);
    }

    #[test]
    fn sorted_values() {
        let ds = DiceSet::from_values([5, 1, 3, 2, 4]).unwrap();
        assert_eq!(ds.sorted_values(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn toggle_hold() {
        let mut ds = DiceSet::new();
        let slot = DiceSlot::new(2).unwrap();
        ds.toggle_hold(slot);
        assert!(ds.is_held(slot));
        ds.toggle_hold(slot);
        assert!(!ds.is_held(slot));
    }

    #[test]
    fn apply_roll_updates_all_faces() {
        let mut ds = DiceSet::from_values([1, 2, 3, 4, 5]).unwrap();
        ds.set_holds(HoldMask::from_array([true, false, true, false, true]));
        let new_faces = [
            FaceValue::new(6).unwrap(),
            FaceValue::new(6).unwrap(),
            FaceValue::new(6).unwrap(),
            FaceValue::new(6).unwrap(),
            FaceValue::new(6).unwrap(),
        ];
        ds.apply_roll(new_faces);
        // All dice updated — held dice get their current face from
        // mark_held_stable, so this is a no-op for them in normal flow.
        assert_eq!(ds.values(), [6, 6, 6, 6, 6]);
    }

    #[test]
    fn clear_holds() {
        let mut ds = DiceSet::new();
        ds.set_holds(HoldMask::all());
        ds.clear_holds();
        assert_eq!(ds.holds().held_count(), 0);
    }

    #[test]
    fn reset() {
        let mut ds = DiceSet::from_values([6, 6, 6, 6, 6]).unwrap();
        ds.set_holds(HoldMask::all());
        ds.reset();
        assert_eq!(ds.values(), [1, 1, 1, 1, 1]);
        assert_eq!(ds.holds().held_count(), 0);
    }
}
