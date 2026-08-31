use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::score::Score;

/// The fixed score for a Full House.
const FULL_HOUSE_SCORE: u32 = 25;

/// The fixed score for a Small Straight.
const SMALL_STRAIGHT_SCORE: u32 = 30;

/// The fixed score for a Large Straight.
const LARGE_STRAIGHT_SCORE: u32 = 40;

/// The fixed score for a Yahtzee (five of a kind).
const YAHTZEE_SCORE: u32 = 50;

/// Calculates the score for a category given the current dice values.
///
/// Returns `Score::ZERO` if the dice combination does not satisfy
/// the category's requirements.
pub fn calculate_score(category: ScoreCategory, dice: &DiceSet) -> Score {
    let values = dice.values();
    let counts = dice.counts();
    let sum = dice.sum();

    let raw = match category {
        ScoreCategory::Ones => counts[1] as u32,
        ScoreCategory::Twos => counts[2] as u32 * 2,
        ScoreCategory::Threes => counts[3] as u32 * 3,
        ScoreCategory::Fours => counts[4] as u32 * 4,
        ScoreCategory::Fives => counts[5] as u32 * 5,
        ScoreCategory::Sixes => counts[6] as u32 * 6,
        ScoreCategory::ThreeOfAKind => {
            if has_n_of_a_kind(&counts, 3) {
                sum
            } else {
                0
            }
        }
        ScoreCategory::FourOfAKind => {
            if has_n_of_a_kind(&counts, 4) {
                sum
            } else {
                0
            }
        }
        ScoreCategory::FullHouse => {
            if is_full_house(&counts) {
                FULL_HOUSE_SCORE
            } else {
                0
            }
        }
        ScoreCategory::SmallStraight => {
            if has_straight(&values, 4) {
                SMALL_STRAIGHT_SCORE
            } else {
                0
            }
        }
        ScoreCategory::LargeStraight => {
            if has_straight(&values, 5) {
                LARGE_STRAIGHT_SCORE
            } else {
                0
            }
        }
        ScoreCategory::Yahtzee => {
            if has_n_of_a_kind(&counts, 5) {
                YAHTZEE_SCORE
            } else {
                0
            }
        }
        ScoreCategory::Chance => sum,
    };

    Score::new(raw)
}

/// Returns true if the dice contain at least `n` dice with the same value.
fn has_n_of_a_kind(counts: &[u8; 7], n: u8) -> bool {
    counts.iter().any(|&c| c >= n)
}

/// Returns true if the dice form a full house (exactly 3 of one value
/// and exactly 2 of another).
fn is_full_house(counts: &[u8; 7]) -> bool {
    let has_three = counts.contains(&3);
    let has_two = counts.contains(&2);
    has_three && has_two
}

/// Returns true if the sorted dice values contain a straight of at
/// least the given length.
///
/// A straight is a sequence of consecutive values (e.g. 2, 3, 4, 5).
fn has_straight(values: &[u8; 5], length: usize) -> bool {
    let sorted = {
        let mut s = *values;
        s.sort_unstable();
        s
    };

    // Deduplicate and count consecutive runs
    let mut max_run = 1usize;
    let mut current_run = 1usize;
    for i in 1..sorted.len() {
        if sorted[i] == sorted[i - 1] {
            // Duplicate, skip
            continue;
        }
        if sorted[i] == sorted[i - 1] + 1 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dice(values: [u8; 5]) -> DiceSet {
        DiceSet::from_values(values).unwrap()
    }

    // --- Upper section ---

    #[test]
    fn ones() {
        let d = dice([1, 1, 3, 4, 6]);
        assert_eq!(calculate_score(ScoreCategory::Ones, &d), Score::new(2));
    }

    #[test]
    fn ones_none() {
        let d = dice([2, 3, 4, 5, 6]);
        assert_eq!(calculate_score(ScoreCategory::Ones, &d), Score::ZERO);
    }

    #[test]
    fn sixes() {
        let d = dice([6, 6, 6, 3, 1]);
        assert_eq!(calculate_score(ScoreCategory::Sixes, &d), Score::new(18));
    }

    // --- Three of a kind ---

    #[test]
    fn three_of_a_kind() {
        let d = dice([5, 5, 5, 3, 1]);
        assert_eq!(calculate_score(ScoreCategory::ThreeOfAKind, &d), Score::new(19));
    }

    #[test]
    fn three_of_a_kind_with_four() {
        let d = dice([4, 4, 4, 4, 2]);
        assert_eq!(calculate_score(ScoreCategory::ThreeOfAKind, &d), Score::new(18));
    }

    #[test]
    fn three_of_a_kind_none() {
        let d = dice([1, 2, 3, 4, 5]);
        assert_eq!(calculate_score(ScoreCategory::ThreeOfAKind, &d), Score::ZERO);
    }

    // --- Four of a kind ---

    #[test]
    fn four_of_a_kind() {
        let d = dice([3, 3, 3, 3, 6]);
        assert_eq!(calculate_score(ScoreCategory::FourOfAKind, &d), Score::new(18));
    }

    #[test]
    fn four_of_a_kind_with_yahtzee() {
        let d = dice([2, 2, 2, 2, 2]);
        assert_eq!(calculate_score(ScoreCategory::FourOfAKind, &d), Score::new(10));
    }

    #[test]
    fn four_of_a_kind_none() {
        let d = dice([1, 1, 2, 3, 4]);
        assert_eq!(calculate_score(ScoreCategory::FourOfAKind, &d), Score::ZERO);
    }

    // --- Full house ---

    #[test]
    fn full_house() {
        let d = dice([3, 3, 3, 5, 5]);
        assert_eq!(calculate_score(ScoreCategory::FullHouse, &d), Score::new(25));
    }

    #[test]
    fn full_house_none_with_four() {
        let d = dice([4, 4, 4, 4, 2]);
        assert_eq!(calculate_score(ScoreCategory::FullHouse, &d), Score::ZERO);
    }

    #[test]
    fn full_house_none_with_yahtzee() {
        let d = dice([6, 6, 6, 6, 6]);
        assert_eq!(calculate_score(ScoreCategory::FullHouse, &d), Score::ZERO);
    }

    // --- Straights ---

    #[test]
    fn small_straight() {
        let d = dice([1, 2, 3, 4, 6]);
        assert_eq!(calculate_score(ScoreCategory::SmallStraight, &d), Score::new(30));
    }

    #[test]
    fn small_straight_with_duplicate() {
        let d = dice([2, 3, 3, 4, 5]);
        assert_eq!(calculate_score(ScoreCategory::SmallStraight, &d), Score::new(30));
    }

    #[test]
    fn small_straight_none() {
        let d = dice([1, 2, 3, 5, 6]);
        assert_eq!(calculate_score(ScoreCategory::SmallStraight, &d), Score::ZERO);
    }

    #[test]
    fn large_straight() {
        let d = dice([2, 3, 4, 5, 6]);
        assert_eq!(calculate_score(ScoreCategory::LargeStraight, &d), Score::new(40));
    }

    #[test]
    fn large_straight_ascending() {
        let d = dice([1, 2, 3, 4, 5]);
        assert_eq!(calculate_score(ScoreCategory::LargeStraight, &d), Score::new(40));
    }

    #[test]
    fn large_straight_none() {
        let d = dice([1, 2, 3, 4, 6]);
        assert_eq!(calculate_score(ScoreCategory::LargeStraight, &d), Score::ZERO);
    }

    // --- Yahtzee ---

    #[test]
    fn yahtzee() {
        let d = dice([5, 5, 5, 5, 5]);
        assert_eq!(calculate_score(ScoreCategory::Yahtzee, &d), Score::new(50));
    }

    #[test]
    fn yahtzee_none() {
        let d = dice([1, 2, 3, 4, 5]);
        assert_eq!(calculate_score(ScoreCategory::Yahtzee, &d), Score::ZERO);
    }

    // --- Chance ---

    #[test]
    fn chance() {
        let d = dice([6, 5, 4, 3, 2]);
        assert_eq!(calculate_score(ScoreCategory::Chance, &d), Score::new(20));
    }

    #[test]
    fn chance_all_same() {
        let d = dice([1, 1, 1, 1, 1]);
        assert_eq!(calculate_score(ScoreCategory::Chance, &d), Score::new(5));
    }
}
