use crate::error::Result;
use crate::error::YatzyError;
use serde::Deserialize;
use serde::Serialize;

/// Index into the player list of a game.
///
/// A newtype wrapper around `usize` that provides type safety for
/// player indexing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerIndex(usize);

impl PlayerIndex {
    /// Create a player index from a raw value.
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Get the raw index value.
    pub fn get(self) -> usize {
        self.0
    }

    /// Returns the next player index, wrapping around to 0.
    pub fn next_wrapping(self, player_count: usize) -> Self {
        Self((self.0 + 1) % player_count)
    }
}

impl std::fmt::Display for PlayerIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validates that a player index is within bounds for the given player count.
pub fn validate_index(index: PlayerIndex, player_count: usize) -> Result<()> {
    if index.get() >= player_count {
        return Err(YatzyError::InvalidPlayerIndex(index.get()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_get() {
        assert_eq!(PlayerIndex::new(3).get(), 3);
    }

    #[test]
    fn next_wrapping() {
        assert_eq!(PlayerIndex::new(0).next_wrapping(4), PlayerIndex::new(1));
        assert_eq!(PlayerIndex::new(3).next_wrapping(4), PlayerIndex::new(0));
    }

    #[test]
    fn validate_index_ok() {
        assert!(validate_index(PlayerIndex::new(2), 4).is_ok());
    }

    #[test]
    fn validate_index_out_of_bounds() {
        assert!(validate_index(PlayerIndex::new(4), 4).is_err());
    }
}
