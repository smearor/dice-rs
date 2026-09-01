use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The classification of a dice roll for celebration purposes.
///
/// Represents special roll combinations that trigger LED celebration
/// effects. Rolls that don't match any special combination are
/// classified as `Normal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RollResult {
    /// Five of a kind — the highest celebration.
    Yahtzee,
    /// Three of one value and two of another.
    FullHouse,
    /// Five consecutive values (1-2-3-4-5 or 2-3-4-5-6).
    LargeStraight,
    /// Four consecutive values.
    SmallStraight,
    /// Four of a kind.
    FourOfAKind,
    /// Three of a kind.
    ThreeOfAKind,
    /// No special combination.
    Normal,
}

impl RollResult {
    /// Returns true if this result warrants a LED celebration effect.
    pub fn is_celebration(self) -> bool {
        !matches!(self, Self::Normal | Self::ThreeOfAKind)
    }

    /// Returns the priority of this result for display purposes.
    /// Higher values indicate more significant results.
    pub fn priority(self) -> u8 {
        match self {
            Self::Yahtzee => 6,
            Self::LargeStraight => 5,
            Self::FullHouse => 4,
            Self::SmallStraight => 3,
            Self::FourOfAKind => 2,
            Self::ThreeOfAKind => 1,
            Self::Normal => 0,
        }
    }
}

impl Display for RollResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yahtzee => write!(f, "Yahtzee!"),
            Self::FullHouse => write!(f, "Full House!"),
            Self::LargeStraight => write!(f, "Große Straße!"),
            Self::SmallStraight => write!(f, "Kleine Straße!"),
            Self::FourOfAKind => write!(f, "Vierlinge!"),
            Self::ThreeOfAKind => write!(f, "Drillinge"),
            Self::Normal => write!(f, "Normal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_celebration_yahtzee() {
        assert!(RollResult::Yahtzee.is_celebration());
    }

    #[test]
    fn is_celebration_full_house() {
        assert!(RollResult::FullHouse.is_celebration());
    }

    #[test]
    fn is_celebration_large_straight() {
        assert!(RollResult::LargeStraight.is_celebration());
    }

    #[test]
    fn is_celebration_small_straight() {
        assert!(RollResult::SmallStraight.is_celebration());
    }

    #[test]
    fn is_celebration_four_of_a_kind() {
        assert!(RollResult::FourOfAKind.is_celebration());
    }

    #[test]
    fn is_not_celebration_three_of_a_kind() {
        assert!(!RollResult::ThreeOfAKind.is_celebration());
    }

    #[test]
    fn is_not_celebration_normal() {
        assert!(!RollResult::Normal.is_celebration());
    }

    #[test]
    fn priority_ordering() {
        assert!(RollResult::Yahtzee.priority() > RollResult::LargeStraight.priority());
        assert!(RollResult::LargeStraight.priority() > RollResult::FullHouse.priority());
        assert!(RollResult::FullHouse.priority() > RollResult::SmallStraight.priority());
        assert!(RollResult::SmallStraight.priority() > RollResult::FourOfAKind.priority());
        assert!(RollResult::FourOfAKind.priority() > RollResult::ThreeOfAKind.priority());
        assert!(RollResult::ThreeOfAKind.priority() > RollResult::Normal.priority());
    }

    #[test]
    fn display_yahtzee() {
        assert_eq!(RollResult::Yahtzee.to_string(), "Yahtzee!");
    }

    #[test]
    fn display_full_house() {
        assert_eq!(RollResult::FullHouse.to_string(), "Full House!");
    }

    #[test]
    fn display_large_straight() {
        assert_eq!(RollResult::LargeStraight.to_string(), "Große Straße!");
    }

    #[test]
    fn display_normal() {
        assert_eq!(RollResult::Normal.to_string(), "Normal");
    }
}
