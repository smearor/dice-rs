use crate::i18n;
use serde::Deserialize;
use serde::Serialize;

/// The section of the scorecard a category belongs to.
///
/// The upper section contains the ones through sixes categories.
/// The lower section contains the combination categories
/// (three-of-a-kind through chance).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CategorySection {
    /// Upper section: ones, twos, threes, fours, fives, sixes.
    Upper,
    /// Lower section: three-of-a-kind, four-of-a-kind, full house,
    /// small straight, large straight, yahtzee, chance.
    Lower,
}

impl CategorySection {
    /// The point threshold for the upper section bonus.
    pub const UPPER_BONUS_THRESHOLD: u32 = 63;

    /// The bonus points awarded when the upper section reaches the threshold.
    pub const UPPER_BONUS_POINTS: u32 = 35;

    /// Returns the Fluent message key for this section.
    pub fn fluent_key(self) -> &'static str {
        match self {
            Self::Upper => "scorecard-upper-section",
            Self::Lower => "scorecard-lower-section",
        }
    }

    /// Returns the localized display name for this section.
    pub fn localized(self) -> String {
        i18n::get(self.fluent_key())
    }
}

impl std::fmt::Display for CategorySection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.localized())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_not_empty() {
        assert!(!CategorySection::Upper.to_string().is_empty());
        assert!(!CategorySection::Lower.to_string().is_empty());
    }

    #[test]
    fn bonus_threshold_is_63() {
        assert_eq!(CategorySection::UPPER_BONUS_THRESHOLD, 63);
    }

    #[test]
    fn bonus_points_is_35() {
        assert_eq!(CategorySection::UPPER_BONUS_POINTS, 35);
    }
}
