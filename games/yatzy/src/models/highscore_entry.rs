use crate::models::score::Score;
use serde::Deserialize;
use serde::Serialize;

/// A single highscore entry recording a player's name and final score.
///
/// Stored as part of `HighscoreList` when a game ends. Entries are
/// sorted by score in descending order and capped at `MAX_ENTRIES`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighscoreEntry {
    /// The player's display name.
    player_name: String,
    /// The player's final grand total score.
    score: Score,
}

impl HighscoreEntry {
    /// Create a new highscore entry.
    pub fn new(player_name: impl Into<String>, score: Score) -> Self {
        Self {
            player_name: player_name.into(),
            score,
        }
    }

    /// Get the player's name.
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    /// Get the score.
    pub fn score(&self) -> Score {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entry_accessors() {
        let entry = HighscoreEntry::new("Alice", Score::new(250));
        assert_eq!(entry.player_name(), "Alice");
        assert_eq!(entry.score(), Score::new(250));
    }
}
