use crate::error::Result;
use crate::error::YahtzeeError;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The number of players in a game (1-6).
///
/// This newtype enforces the invariant that a valid game has
/// between 1 and 6 players, matching the 6 default player colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerCount(u8);

impl PlayerCount {
    /// The minimum number of players (1).
    pub const MIN: u8 = 1;

    /// The maximum number of players (6).
    pub const MAX: u8 = 6;

    /// Create a player count. Returns an error if the value is not in [1, 6].
    pub fn new(value: u8) -> Result<Self> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(YahtzeeError::InvalidPlayerCount(value));
        }
        Ok(Self(value))
    }

    /// Get the raw count value.
    pub fn get(self) -> usize {
        self.0 as usize
    }

    /// Returns true if this is a single-player game.
    pub fn is_single_player(self) -> bool {
        self.0 == 1
    }

    /// Returns true if this is a multi-player game.
    pub fn is_multi_player(self) -> bool {
        self.0 > 1
    }
}

impl Display for PlayerCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(PlayerCount::new(1).unwrap().get(), 1);
        assert_eq!(PlayerCount::new(6).unwrap().get(), 6);
        assert_eq!(PlayerCount::new(3).unwrap().get(), 3);
    }

    #[test]
    fn new_zero_fails() {
        assert!(PlayerCount::new(0).is_err());
    }

    #[test]
    fn new_too_many_fails() {
        assert!(PlayerCount::new(7).is_err());
    }

    #[test]
    fn is_single_player() {
        assert!(PlayerCount::new(1).unwrap().is_single_player());
        assert!(!PlayerCount::new(2).unwrap().is_single_player());
    }

    #[test]
    fn is_multi_player() {
        assert!(PlayerCount::new(2).unwrap().is_multi_player());
        assert!(!PlayerCount::new(1).unwrap().is_multi_player());
    }

    #[test]
    fn display() {
        assert_eq!(PlayerCount::new(3).unwrap().to_string(), "3");
    }
}
