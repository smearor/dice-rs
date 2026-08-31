use crate::error::Result;
use crate::error::YahtzeeError;
use serde::Deserialize;
use serde::Serialize;

/// How many rolls the current player has used in this turn (1-3).
///
/// A newtype wrapper around `u8` that enforces the invariant
/// that a roll count is in the range 1-3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RollCount(u8);

impl RollCount {
    /// The maximum number of rolls per turn in Kniffel.
    pub const MAX: u8 = 3;

    /// The first roll of a turn.
    pub const FIRST: Self = Self(1);

    /// Create a roll count. Returns an error if the value is not 1-3.
    pub fn new(value: u8) -> Result<Self> {
        if value == 0 || value > Self::MAX {
            return Err(YahtzeeError::InvalidRollCount(value));
        }
        Ok(Self(value))
    }

    /// Get the raw roll count value.
    pub fn get(self) -> u8 {
        self.0
    }

    /// Returns the next roll count, or an error if already at max.
    pub fn increment(self) -> Result<Self> {
        if self.0 >= Self::MAX {
            return Err(YahtzeeError::InvalidRollCount(self.0 + 1));
        }
        Ok(Self(self.0 + 1))
    }

    /// Returns true if this is the last allowed roll.
    pub fn is_last(self) -> bool {
        self.0 == Self::MAX
    }

    /// Returns true if no more rolls are allowed.
    pub fn is_exhausted(self) -> bool {
        self.0 >= Self::MAX
    }
}

impl std::fmt::Display for RollCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, Self::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(RollCount::new(1).unwrap().get(), 1);
        assert_eq!(RollCount::new(3).unwrap().get(), 3);
    }

    #[test]
    fn new_zero_fails() {
        assert!(RollCount::new(0).is_err());
    }

    #[test]
    fn new_over_max_fails() {
        assert!(RollCount::new(4).is_err());
    }

    #[test]
    fn increment() {
        let r1 = RollCount::FIRST;
        let r2 = r1.increment().unwrap();
        assert_eq!(r2.get(), 2);
        let r3 = r2.increment().unwrap();
        assert_eq!(r3.get(), 3);
    }

    #[test]
    fn increment_at_max_fails() {
        let r3 = RollCount::new(3).unwrap();
        assert!(r3.increment().is_err());
    }

    #[test]
    fn is_last() {
        assert!(!RollCount::FIRST.is_last());
        assert!(RollCount::new(3).unwrap().is_last());
    }

    #[test]
    fn is_exhausted() {
        assert!(!RollCount::FIRST.is_exhausted());
        assert!(RollCount::new(3).unwrap().is_exhausted());
    }

    #[test]
    fn display() {
        assert_eq!(RollCount::FIRST.to_string(), "1/3");
        assert_eq!(RollCount::new(3).unwrap().to_string(), "3/3");
    }
}
