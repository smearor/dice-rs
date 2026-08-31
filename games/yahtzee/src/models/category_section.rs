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
}

impl std::fmt::Display for CategorySection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Upper => write!(f, "Obere Hälfte"),
            Self::Lower => write!(f, "Untere Hälfte"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_german() {
        assert_eq!(CategorySection::Upper.to_string(), "Obere Hälfte");
        assert_eq!(CategorySection::Lower.to_string(), "Untere Hälfte");
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
