use crate::models::score::Score;
use serde::Deserialize;
use serde::Serialize;

/// The state of a single scorecard category entry.
///
/// A category starts empty. Once a player enters a score or crosses out
/// the category, it becomes filled or crossed out respectively. Both
/// filled and crossed-out categories are considered "used" and cannot
/// be changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ScoreEntry {
    /// The category has not been filled yet.
    #[default]
    Empty,
    /// The category has been filled with a score.
    Filled(Score),
    /// The category has been crossed out (score 0, no valid combination).
    CrossedOut,
}

impl ScoreEntry {
    /// Returns true if the category is empty (available for entry).
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns true if the category has been used (filled or crossed out).
    pub fn is_used(&self) -> bool {
        !self.is_empty()
    }

    /// Returns the score value of this entry, or 0 if empty or crossed out.
    pub fn score(&self) -> Score {
        match self {
            Self::Filled(score) => *score,
            Self::Empty | Self::CrossedOut => Score::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_empty() {
        assert!(ScoreEntry::Empty.is_empty());
        assert!(!ScoreEntry::Empty.is_used());
    }

    #[test]
    fn filled_is_used() {
        let entry = ScoreEntry::Filled(Score::new(30));
        assert!(!entry.is_empty());
        assert!(entry.is_used());
        assert_eq!(entry.score(), Score::new(30));
    }

    #[test]
    fn crossed_out_is_used() {
        assert!(!ScoreEntry::CrossedOut.is_empty());
        assert!(ScoreEntry::CrossedOut.is_used());
        assert_eq!(ScoreEntry::CrossedOut.score(), Score::ZERO);
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(ScoreEntry::default(), ScoreEntry::Empty);
    }
}
