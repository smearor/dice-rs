use crate::models::category_section::CategorySection;
use serde::Deserialize;
use serde::Serialize;

/// The 13 Kniffel scorecard categories.
///
/// Each category has specific rules for what dice combination is required
/// and how the score is calculated. Categories are divided into the
/// upper section (ones through sixes) and the lower section
/// (three-of-a-kind through chance).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoreCategory {
    /// Sum of all 1s rolled.
    Ones,
    /// Sum of all 2s rolled.
    Twos,
    /// Sum of all 3s rolled.
    Threes,
    /// Sum of all 4s rolled.
    Fours,
    /// Sum of all 5s rolled.
    Fives,
    /// Sum of all 6s rolled.
    Sixes,
    /// At least three dice showing the same value. Score: sum of all dice.
    ThreeOfAKind,
    /// At least four dice showing the same value. Score: sum of all dice.
    FourOfAKind,
    /// Three of one value plus two of another. Score: 25 points.
    FullHouse,
    /// Four consecutive values. Score: 30 points.
    SmallStraight,
    /// Five consecutive values. Score: 40 points.
    LargeStraight,
    /// All five dice showing the same value. Score: 50 points.
    Yahtzee,
    /// Any combination. Score: sum of all dice.
    Chance,
}

impl ScoreCategory {
    /// Returns all 13 categories in scorecard order (upper section first).
    pub const ALL: [Self; 13] = [
        Self::Ones,
        Self::Twos,
        Self::Threes,
        Self::Fours,
        Self::Fives,
        Self::Sixes,
        Self::ThreeOfAKind,
        Self::FourOfAKind,
        Self::FullHouse,
        Self::SmallStraight,
        Self::LargeStraight,
        Self::Yahtzee,
        Self::Chance,
    ];

    /// Returns the section (upper or lower) this category belongs to.
    pub fn section(&self) -> CategorySection {
        match self {
            Self::Ones | Self::Twos | Self::Threes | Self::Fours | Self::Fives | Self::Sixes => CategorySection::Upper,
            Self::ThreeOfAKind | Self::FourOfAKind | Self::FullHouse | Self::SmallStraight | Self::LargeStraight | Self::Yahtzee | Self::Chance => {
                CategorySection::Lower
            }
        }
    }

    /// Returns the index of this category within the full scorecard (0-12).
    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|c| c == self).unwrap()
    }
}

impl std::fmt::Display for ScoreCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ones => write!(f, "Einser"),
            Self::Twos => write!(f, "Zweier"),
            Self::Threes => write!(f, "Dreier"),
            Self::Fours => write!(f, "Vierer"),
            Self::Fives => write!(f, "Fünfer"),
            Self::Sixes => write!(f, "Sechser"),
            Self::ThreeOfAKind => write!(f, "Dreierpasch"),
            Self::FourOfAKind => write!(f, "Viererpasch"),
            Self::FullHouse => write!(f, "Full House"),
            Self::SmallStraight => write!(f, "Kleine Straße"),
            Self::LargeStraight => write!(f, "Große Straße"),
            Self::Yahtzee => write!(f, "Kniffel"),
            Self::Chance => write!(f, "Chance"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_has_13_categories() {
        assert_eq!(ScoreCategory::ALL.len(), 13);
    }

    #[test]
    fn section_upper() {
        assert_eq!(ScoreCategory::Ones.section(), CategorySection::Upper);
        assert_eq!(ScoreCategory::Sixes.section(), CategorySection::Upper);
    }

    #[test]
    fn section_lower() {
        assert_eq!(ScoreCategory::ThreeOfAKind.section(), CategorySection::Lower);
        assert_eq!(ScoreCategory::Chance.section(), CategorySection::Lower);
    }

    #[test]
    fn index_is_consistent() {
        for (i, cat) in ScoreCategory::ALL.iter().enumerate() {
            assert_eq!(cat.index(), i);
        }
    }

    #[test]
    fn display_german() {
        assert_eq!(ScoreCategory::Ones.to_string(), "Einser");
        assert_eq!(ScoreCategory::Yahtzee.to_string(), "Kniffel");
        assert_eq!(ScoreCategory::FullHouse.to_string(), "Full House");
    }
}
