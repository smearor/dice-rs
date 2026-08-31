use crate::error::Result;
use crate::error::YahtzeeError;
use serde::Deserialize;
use serde::Serialize;

/// A slot index for a die in the dice set (0-4).
///
/// A newtype wrapper around `u8` that enforces the invariant
/// that a dice slot is in the range 0-4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiceSlot(u8);

impl DiceSlot {
    /// The number of dice in a standard Kniffel game.
    pub const COUNT: usize = 5;

    /// Create a dice slot. Returns an error if the value is >= 5.
    pub fn new(value: u8) -> Result<Self> {
        if value as usize >= Self::COUNT {
            return Err(YahtzeeError::InvalidDiceSlot(value));
        }
        Ok(Self(value))
    }

    /// Get the raw slot value.
    pub fn get(self) -> u8 {
        self.0
    }

    /// Returns all valid dice slots (0, 1, 2, 3, 4).
    pub fn all() -> impl Iterator<Item = Self> {
        (0..Self::COUNT as u8).map(Self)
    }
}

impl std::fmt::Display for DiceSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(DiceSlot::new(0).unwrap().get(), 0);
        assert_eq!(DiceSlot::new(4).unwrap().get(), 4);
    }

    #[test]
    fn new_out_of_range() {
        assert!(DiceSlot::new(5).is_err());
        assert!(DiceSlot::new(255).is_err());
    }

    #[test]
    fn all_returns_5_slots() {
        assert_eq!(DiceSlot::all().count(), 5);
    }

    #[test]
    fn count_is_5() {
        assert_eq!(DiceSlot::COUNT, 5);
    }
}
