use crate::i18n;
use crate::error::Result;
use crate::error::YahtzeeError;
use serde::Deserialize;
use serde::Serialize;

/// The current round number in a Kniffel game (1-13).
///
/// A newtype wrapper around `u8` that enforces the invariant
/// that a round number is in the range 1-13.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoundNumber(u8);

impl RoundNumber {
    /// The total number of rounds in a Kniffel game.
    pub const TOTAL: u8 = 13;

    /// The first round of a game.
    pub const FIRST: Self = Self(1);

    /// The last round of a game.
    pub const LAST: Self = Self(13);

    /// Create a round number. Returns an error if the value is not 1-13.
    pub fn new(value: u8) -> Result<Self> {
        if value == 0 || value > Self::TOTAL {
            return Err(YahtzeeError::InvalidRoundNumber(value));
        }
        Ok(Self(value))
    }

    /// Get the raw round number value.
    pub fn get(self) -> u8 {
        self.0
    }

    /// Returns the next round number, or an error if already at the last.
    pub fn increment(self) -> Result<Self> {
        if self.0 >= Self::TOTAL {
            return Err(YahtzeeError::InvalidRoundNumber(self.0 + 1));
        }
        Ok(Self(self.0 + 1))
    }

    /// Returns true if this is the last round.
    pub fn is_last(self) -> bool {
        self.0 == Self::TOTAL
    }
}

impl std::fmt::Display for RoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", i18n::get_int_int("round-number", "current", self.0 as i64, "total", Self::TOTAL as i64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(RoundNumber::new(1).unwrap().get(), 1);
        assert_eq!(RoundNumber::new(13).unwrap().get(), 13);
    }

    #[test]
    fn new_zero_fails() {
        assert!(RoundNumber::new(0).is_err());
    }

    #[test]
    fn new_over_total_fails() {
        assert!(RoundNumber::new(14).is_err());
    }

    #[test]
    fn increment() {
        let r1 = RoundNumber::FIRST;
        let r2 = r1.increment().unwrap();
        assert_eq!(r2.get(), 2);
    }

    #[test]
    fn increment_at_last_fails() {
        let r13 = RoundNumber::LAST;
        assert!(r13.increment().is_err());
    }

    #[test]
    fn is_last() {
        assert!(!RoundNumber::FIRST.is_last());
        assert!(RoundNumber::LAST.is_last());
    }

    #[test]
    fn display() {
        assert_eq!(RoundNumber::FIRST.to_string(), i18n::get_int_int("round-number", "current", 1, "total", 13));
        assert_eq!(RoundNumber::LAST.to_string(), i18n::get_int_int("round-number", "current", 13, "total", 13));
    }
}
