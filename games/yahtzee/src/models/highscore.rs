use crate::models::score::Score;
use serde::Deserialize;
use serde::Serialize;

/// Maximum number of highscore entries kept on disk.
const MAX_ENTRIES: usize = 20;

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

/// A persisted list of highscore entries.
///
/// Serialized to JSON and stored on disk so that highscores survive
/// across game sessions. Entries are sorted by score descending.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighscoreList {
    /// The highscore entries, sorted by score descending.
    entries: Vec<HighscoreEntry>,
}

impl HighscoreList {
    /// Create a new empty highscore list.
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Get the highscore entries.
    pub fn entries(&self) -> &[HighscoreEntry] {
        &self.entries
    }

    /// Add a new entry to the highscore list.
    ///
    /// The entry is inserted in the correct position to maintain
    /// descending score order. The list is then trimmed to `MAX_ENTRIES`.
    pub fn add(&mut self, entry: HighscoreEntry) {
        let pos = self.entries.iter().position(|e| e.score < entry.score).unwrap_or(self.entries.len());
        self.entries.insert(pos, entry);
        if self.entries.len() > MAX_ENTRIES {
            self.entries.truncate(MAX_ENTRIES);
        }
    }

    /// Returns the highest score in the list, or `None` if empty.
    pub fn top_score(&self) -> Option<Score> {
        self.entries.first().map(|e| e.score)
    }

    /// Returns the number of entries in the list.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for HighscoreList {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn new_list_is_empty() {
        let list = HighscoreList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn add_inserts_in_descending_order() {
        let mut list = HighscoreList::new();
        list.add(HighscoreEntry::new("Alice", Score::new(100)));
        list.add(HighscoreEntry::new("Bob", Score::new(300)));
        list.add(HighscoreEntry::new("Carol", Score::new(200)));
        assert_eq!(list.entries()[0].player_name(), "Bob");
        assert_eq!(list.entries()[1].player_name(), "Carol");
        assert_eq!(list.entries()[2].player_name(), "Alice");
    }

    #[test]
    fn add_trims_to_max_entries() {
        let mut list = HighscoreList::new();
        for i in 0..25 {
            list.add(HighscoreEntry::new(format!("Player{i}"), Score::new(i as u32)));
        }
        assert_eq!(list.len(), 20);
        // Highest scores should be kept
        assert_eq!(list.entries()[0].score(), Score::new(24));
        assert_eq!(list.entries()[19].score(), Score::new(5));
    }

    #[test]
    fn top_score_returns_highest() {
        let mut list = HighscoreList::new();
        assert!(list.top_score().is_none());
        list.add(HighscoreEntry::new("Alice", Score::new(100)));
        assert_eq!(list.top_score(), Some(Score::new(100)));
        list.add(HighscoreEntry::new("Bob", Score::new(300)));
        assert_eq!(list.top_score(), Some(Score::new(300)));
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let mut list = HighscoreList::new();
        list.add(HighscoreEntry::new("Alice", Score::new(300)));
        list.add(HighscoreEntry::new("Bob", Score::new(200)));
        let json = serde_json::to_string(&list).unwrap();
        let deserialized: HighscoreList = serde_json::from_str(&json).unwrap();
        assert_eq!(list, deserialized);
    }

    #[test]
    fn default_is_empty() {
        let list = HighscoreList::default();
        assert!(list.is_empty());
    }
}
