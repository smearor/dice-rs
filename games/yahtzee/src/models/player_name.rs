use crate::error::Result;
use crate::error::YahtzeeError;
use serde::Deserialize;
use serde::Serialize;

/// A player's display name.
///
/// A newtype wrapper around `String` that enforces the invariant
/// that a player name is non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerName(String);

impl PlayerName {
    /// Create a player name. Returns an error if the name is empty
    /// or contains only whitespace.
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(YahtzeeError::EmptyPlayerName);
        }
        Ok(Self(name))
    }

    /// Get the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PlayerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        let name = PlayerName::new("Alice").unwrap();
        assert_eq!(name.as_str(), "Alice");
    }

    #[test]
    fn new_empty_fails() {
        assert!(PlayerName::new("").is_err());
    }

    #[test]
    fn new_whitespace_only_fails() {
        assert!(PlayerName::new("   ").is_err());
    }

    #[test]
    fn display() {
        let name = PlayerName::new("Bob").unwrap();
        assert_eq!(name.to_string(), "Bob");
    }
}
