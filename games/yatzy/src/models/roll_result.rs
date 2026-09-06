use crate::i18n::Localized;
use crate::impl_display_localized;
use serde::Deserialize;
use serde::Serialize;

/// The classification of a dice roll for celebration purposes.
///
/// Represents special roll combinations that trigger LED celebration
/// effects. Rolls that don't match any special combination are
/// classified as `Normal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RollResult {
    /// Five of a kind — the highest celebration.
    Yatzy,
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
            Self::Yatzy => 6,
            Self::LargeStraight => 5,
            Self::FullHouse => 4,
            Self::SmallStraight => 3,
            Self::FourOfAKind => 2,
            Self::ThreeOfAKind => 1,
            Self::Normal => 0,
        }
    }
}

impl Localized for RollResult {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::Yatzy => "roll-result-yatzy",
            Self::FullHouse => "roll-result-full-house",
            Self::LargeStraight => "roll-result-large-straight",
            Self::SmallStraight => "roll-result-small-straight",
            Self::FourOfAKind => "roll-result-four-of-a-kind",
            Self::ThreeOfAKind => "roll-result-three-of-a-kind",
            Self::Normal => "roll-result-normal",
        }
    }
}

impl_display_localized!(RollResult);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_celebration_yatzy() {
        assert!(RollResult::Yatzy.is_celebration());
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
        assert!(RollResult::Yatzy.priority() > RollResult::LargeStraight.priority());
        assert!(RollResult::LargeStraight.priority() > RollResult::FullHouse.priority());
        assert!(RollResult::FullHouse.priority() > RollResult::SmallStraight.priority());
        assert!(RollResult::SmallStraight.priority() > RollResult::FourOfAKind.priority());
        assert!(RollResult::FourOfAKind.priority() > RollResult::ThreeOfAKind.priority());
        assert!(RollResult::ThreeOfAKind.priority() > RollResult::Normal.priority());
    }

    #[test]
    fn display_yatzy() {
        assert_eq!(RollResult::Yatzy.to_string(), RollResult::Yatzy.localized());
    }

    #[test]
    fn display_full_house() {
        assert_eq!(RollResult::FullHouse.to_string(), RollResult::FullHouse.localized());
    }

    #[test]
    fn display_large_straight() {
        assert_eq!(RollResult::LargeStraight.to_string(), RollResult::LargeStraight.localized());
    }

    #[test]
    fn display_normal() {
        assert_eq!(RollResult::Normal.to_string(), RollResult::Normal.localized());
    }
}
