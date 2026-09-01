use crate::models::dice_set::DiceSet;
use crate::models::roll_result::RollResult;
use crate::services::led_effect::LedEffect;

/// Detects special roll combinations and maps them to LED celebration effects.
///
/// The detector analyzes a `DiceSet` after a roll completes, classifies
/// the result, and returns the appropriate `LedEffect` for celebration.
/// Non-celebration rolls return `None`.
pub struct CelebrationDetector;

impl CelebrationDetector {
    /// Classify a dice set into a `RollResult`.
    pub fn classify(dice: &DiceSet) -> RollResult {
        let counts = dice.counts();
        let values = dice.values();

        if Self::has_n_of_a_kind(&counts, 5) {
            return RollResult::Yahtzee;
        }
        if Self::has_straight(&values, 5) {
            return RollResult::LargeStraight;
        }
        if Self::is_full_house(&counts) {
            return RollResult::FullHouse;
        }
        if Self::has_straight(&values, 4) {
            return RollResult::SmallStraight;
        }
        if Self::has_n_of_a_kind(&counts, 4) {
            return RollResult::FourOfAKind;
        }
        if Self::has_n_of_a_kind(&counts, 3) {
            return RollResult::ThreeOfAKind;
        }
        RollResult::Normal
    }

    /// Returns the LED celebration effect for a roll result, if any.
    pub fn effect_for(result: RollResult) -> Option<LedEffect> {
        match result {
            RollResult::Yahtzee => Some(LedEffect::yahtzee()),
            RollResult::FullHouse => Some(LedEffect::full_house()),
            RollResult::LargeStraight => Some(LedEffect::large_straight()),
            RollResult::SmallStraight => Some(LedEffect::Celebrate {
                pulse_count: 2,
                color: crate::models::player_color::PlayerColor::CYAN,
            }),
            RollResult::FourOfAKind => Some(LedEffect::Celebrate {
                pulse_count: 2,
                color: crate::models::player_color::PlayerColor::ORANGE,
            }),
            RollResult::ThreeOfAKind | RollResult::Normal => None,
        }
    }

    /// Detect and return the celebration effect for a dice set.
    ///
    /// Convenience method combining `classify` and `effect_for`.
    pub fn detect(dice: &DiceSet) -> Option<LedEffect> {
        let result = Self::classify(dice);
        Self::effect_for(result)
    }

    /// Returns true if the dice contain at least `n` dice with the same value.
    fn has_n_of_a_kind(counts: &[u8; 7], n: u8) -> bool {
        counts.iter().any(|&c| c >= n)
    }

    /// Returns true if the dice form a full house (3 + 2).
    fn is_full_house(counts: &[u8; 7]) -> bool {
        let has_three = counts.contains(&3);
        let has_two = counts.contains(&2);
        has_three && has_two
    }

    /// Returns true if the dice contain a straight of at least `length` consecutive values.
    fn has_straight(values: &[u8; 5], length: u8) -> bool {
        let unique: Vec<u8> = {
            let mut v = values.to_vec();
            v.sort();
            v.dedup();
            v
        };
        if unique.len() < length as usize {
            return false;
        }
        let mut max_run: u8 = 1;
        let mut current_run: u8 = 1;
        for i in 1..unique.len() {
            if unique[i] == unique[i - 1] + 1 {
                current_run += 1;
                if current_run > max_run {
                    max_run = current_run;
                }
            } else {
                current_run = 1;
            }
        }
        max_run >= length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dice(values: [u8; 5]) -> DiceSet {
        DiceSet::from_values(values).unwrap_or_default()
    }

    #[test]
    fn classify_yahtzee() {
        let d = dice([5, 5, 5, 5, 5]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::Yahtzee);
    }

    #[test]
    fn classify_large_straight() {
        let d = dice([1, 2, 3, 4, 5]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::LargeStraight);
        let d = dice([2, 3, 4, 5, 6]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::LargeStraight);
    }

    #[test]
    fn classify_full_house() {
        let d = dice([3, 3, 3, 6, 6]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::FullHouse);
    }

    #[test]
    fn classify_small_straight() {
        let d = dice([1, 2, 3, 4, 6]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::SmallStraight);
    }

    #[test]
    fn classify_four_of_a_kind() {
        let d = dice([4, 4, 4, 4, 2]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::FourOfAKind);
    }

    #[test]
    fn classify_three_of_a_kind() {
        let d = dice([3, 3, 3, 5, 6]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::ThreeOfAKind);
    }

    #[test]
    fn classify_normal() {
        let d = dice([1, 2, 3, 5, 6]);
        assert_eq!(CelebrationDetector::classify(&d), RollResult::Normal);
    }

    #[test]
    fn effect_for_yahtzee() {
        let effect = CelebrationDetector::effect_for(RollResult::Yahtzee);
        assert!(effect.is_some());
        assert_eq!(effect.unwrap(), LedEffect::yahtzee());
    }

    #[test]
    fn effect_for_normal_none() {
        assert!(CelebrationDetector::effect_for(RollResult::Normal).is_none());
    }

    #[test]
    fn effect_for_three_of_a_kind_none() {
        assert!(CelebrationDetector::effect_for(RollResult::ThreeOfAKind).is_none());
    }

    #[test]
    fn detect_yahtzee() {
        let d = dice([6, 6, 6, 6, 6]);
        assert!(CelebrationDetector::detect(&d).is_some());
    }

    #[test]
    fn detect_normal_none() {
        let d = dice([1, 2, 3, 5, 6]);
        assert!(CelebrationDetector::detect(&d).is_none());
    }
}
