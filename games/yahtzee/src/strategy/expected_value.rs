use crate::error::Result;
use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use crate::models::roll_count::RollCount;
use crate::models::score::Score;
use crate::models::scorecard::Scorecard;
use crate::rules::scoring::calculate_score;
use crate::strategy::probability::Probability;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

/// An expected value (EV) representing the average score outcome.
///
/// This newtype wraps an f64 and is used throughout the strategy module
/// for comparing hold decisions and category choices.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ExpectedValue(f64);

impl ExpectedValue {
    /// Create an expected value from a raw f64.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Zero expected value.
    pub const ZERO: Self = Self(0.0);

    /// Get the raw f64 value.
    pub fn get(self) -> f64 {
        self.0
    }
}

impl Add for ExpectedValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Display for ExpectedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

/// A hold decision recommendation with its expected value.
///
/// Represents the decision to hold certain dice and re-roll the rest,
/// along with the expected score if the resulting dice are entered
/// into the best available category.
#[derive(Debug, Clone, PartialEq)]
pub struct HoldDecision {
    /// The hold mask to apply.
    holds: HoldMask,
    /// The expected value of this hold decision.
    expected_value: ExpectedValue,
}

impl HoldDecision {
    /// Create a new hold decision.
    pub fn new(holds: HoldMask, expected_value: ExpectedValue) -> Self {
        Self { holds, expected_value }
    }

    /// Get the hold mask.
    pub fn holds(&self) -> HoldMask {
        self.holds
    }

    /// Get the expected value.
    pub fn expected_value(&self) -> ExpectedValue {
        self.expected_value
    }
}

/// A category recommendation with its expected value.
///
/// Represents the decision to enter the current dice into a specific
/// category, along with the score that would be achieved.
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryChoice {
    /// The category to enter.
    category: ScoreCategory,
    /// The score that would be achieved.
    score: Score,
}

impl CategoryChoice {
    /// Create a new category choice.
    pub fn new(category: ScoreCategory, score: Score) -> Self {
        Self { category, score }
    }

    /// Get the category.
    pub fn category(&self) -> ScoreCategory {
        self.category
    }

    /// Get the score.
    pub fn score(&self) -> Score {
        self.score
    }
}

/// Calculate the expected value of re-rolling with the given hold mask
/// for a specific category.
///
/// The EV is: P(score > 0) * average_positive_score.
/// For simplicity with exact enumeration, we compute:
/// sum(score for each outcome) / total_outcomes.
pub fn expected_value_for_category(current: &DiceSet, holds: HoldMask, category: ScoreCategory) -> Result<ExpectedValue> {
    let reroll_count = holds.reroll_count();
    if reroll_count == 0 {
        let score = calculate_score(category, current);
        return Ok(ExpectedValue::new(score.get() as f64));
    }

    // Exact enumeration for up to 5 dice
    if reroll_count <= 5 {
        let total = 6u32.pow(reroll_count as u32);
        let mut total_score: f64 = 0.0;

        let held_values: Vec<u8> = DiceSlot::all()
            .filter(|slot| holds.is_held(*slot))
            .map(|slot| current.values()[slot.get() as usize])
            .collect();

        let mut reroll = vec![1u8; reroll_count];
        enumerate_reroll_ev(&mut reroll, 0, &held_values, category, &mut total_score);

        return Ok(ExpectedValue::new(total_score / total as f64));
    }

    Ok(ExpectedValue::ZERO)
}

/// Enumerate re-roll outcomes and accumulate total score.
fn enumerate_reroll_ev(reroll: &mut [u8], pos: usize, held_values: &[u8], category: ScoreCategory, total_score: &mut f64) {
    if pos == reroll.len() {
        let mut all_values = held_values.to_vec();
        all_values.extend_from_slice(reroll);
        if all_values.len() == 5
            && let Ok(dice) = DiceSet::from_values([all_values[0], all_values[1], all_values[2], all_values[3], all_values[4]])
        {
            let score = calculate_score(category, &dice);
            *total_score += score.get() as f64;
        }
        return;
    }
    for face in 1..=6 {
        reroll[pos] = face;
        enumerate_reroll_ev(reroll, pos + 1, held_values, category, total_score);
    }
}

/// Find the best hold decision for the current dice and scorecard.
///
/// Evaluates all possible hold masks (2^5 = 32) and returns the one
/// with the highest expected value across all empty categories.
pub fn best_hold_decision(current: &DiceSet, scorecard: &Scorecard, rolls_remaining: RollCount) -> Result<HoldDecision> {
    if rolls_remaining.is_exhausted() {
        // No re-rolls left — must enter a score
        let best_category = best_category_choice(current, scorecard);
        return Ok(HoldDecision::new(HoldMask::all(), ExpectedValue::new(best_category.score().get() as f64)));
    }

    let mut best = HoldDecision::new(HoldMask::none(), ExpectedValue::ZERO);

    // Enumerate all 32 hold masks
    for mask in 0..32u8 {
        let held = [mask & 1 != 0, mask & 2 != 0, mask & 4 != 0, mask & 8 != 0, mask & 16 != 0];
        let holds = HoldMask::from_array(held);

        // Calculate EV across all empty categories
        let mut best_ev = ExpectedValue::ZERO;
        for category in scorecard.empty_categories() {
            let ev = expected_value_for_category(current, holds, category)?;
            if ev.get() > best_ev.get() {
                best_ev = ev;
            }
        }

        let decision = HoldDecision::new(holds, best_ev);
        if decision.expected_value().get() > best.expected_value().get() {
            best = decision;
        }
    }

    Ok(best)
}

/// Find the best category to enter the current dice into.
///
/// Returns the empty category with the highest score.
pub fn best_category_choice(current: &DiceSet, scorecard: &Scorecard) -> CategoryChoice {
    let mut best = CategoryChoice::new(ScoreCategory::Chance, Score::ZERO);

    for category in scorecard.empty_categories() {
        let score = calculate_score(category, current);
        if score.get() > best.score().get() {
            best = CategoryChoice::new(category, score);
        }
    }

    best
}

/// Calculate the probability of improving the score by re-rolling.
///
/// Returns the probability that re-rolling with the given hold mask
/// will produce a higher score in the best category than the current dice.
pub fn improvement_probability(current: &DiceSet, holds: HoldMask, scorecard: &Scorecard) -> Result<Probability> {
    let current_best = best_category_choice(current, scorecard);
    let reroll_count = holds.reroll_count();

    if reroll_count == 0 {
        return Ok(Probability::ZERO);
    }

    if reroll_count <= 5 {
        let total = 6u32.pow(reroll_count as u32);
        let mut favorable: u32 = 0;

        let held_values: Vec<u8> = DiceSlot::all()
            .filter(|slot| holds.is_held(*slot))
            .map(|slot| current.values()[slot.get() as usize])
            .collect();

        let mut reroll = vec![1u8; reroll_count];
        enumerate_improvement(&mut reroll, 0, &held_values, scorecard, current_best.score(), &mut favorable);

        return Probability::from_fraction(favorable, total);
    }

    Ok(Probability::ZERO)
}

/// Enumerate re-roll outcomes and count improvements.
fn enumerate_improvement(reroll: &mut [u8], pos: usize, held_values: &[u8], scorecard: &Scorecard, current_best_score: Score, favorable: &mut u32) {
    if pos == reroll.len() {
        let mut all_values = held_values.to_vec();
        all_values.extend_from_slice(reroll);
        if all_values.len() == 5
            && let Ok(dice) = DiceSet::from_values([all_values[0], all_values[1], all_values[2], all_values[3], all_values[4]])
        {
            let new_best = best_category_choice(&dice, scorecard);
            if new_best.score().get() > current_best_score.get() {
                *favorable += 1;
            }
        }
        return;
    }
    for face in 1..=6 {
        reroll[pos] = face;
        enumerate_improvement(reroll, pos + 1, held_values, scorecard, current_best_score, favorable);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::player::Player;
    use crate::models::player_color::PlayerColor;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    fn make_scorecard() -> Scorecard {
        let player = Player::new(PlayerName::new("Test").unwrap(), PlayerColor::RED, PlayerType::Human);
        player.scorecard().clone()
    }

    fn dice(values: [u8; 5]) -> DiceSet {
        DiceSet::from_values(values).unwrap()
    }

    #[test]
    fn expected_value_new() {
        let ev = ExpectedValue::new(15.5);
        assert_eq!(ev.get(), 15.5);
    }

    #[test]
    fn expected_value_zero() {
        assert_eq!(ExpectedValue::ZERO.get(), 0.0);
    }

    #[test]
    fn expected_value_add() {
        let a = ExpectedValue::new(10.0);
        let b = ExpectedValue::new(5.0);
        assert_eq!((a + b).get(), 15.0);
    }

    #[test]
    fn expected_value_display() {
        let ev = ExpectedValue::new(12.5);
        assert_eq!(ev.to_string(), "12.5");
    }

    #[test]
    fn hold_decision_new() {
        let holds = HoldMask::none();
        let ev = ExpectedValue::new(10.0);
        let decision = HoldDecision::new(holds, ev);
        assert_eq!(decision.holds(), holds);
        assert_eq!(decision.expected_value(), ev);
    }

    #[test]
    fn category_choice_new() {
        let choice = CategoryChoice::new(ScoreCategory::Ones, Score::new(3));
        assert_eq!(choice.category(), ScoreCategory::Ones);
        assert_eq!(choice.score(), Score::new(3));
    }

    #[test]
    fn best_category_choice_yahtzee() {
        let d = dice([5, 5, 5, 5, 5]);
        let scorecard = make_scorecard();
        let best = best_category_choice(&d, &scorecard);
        assert_eq!(best.category(), ScoreCategory::Yahtzee);
        assert_eq!(best.score(), Score::new(50));
    }

    #[test]
    fn best_category_choice_chance_when_nothing_else() {
        let d = dice([1, 2, 3, 5, 6]);
        let scorecard = make_scorecard();
        let best = best_category_choice(&d, &scorecard);
        // Chance scores 17, which is the highest available
        assert_eq!(best.category(), ScoreCategory::Chance);
        assert_eq!(best.score(), Score::new(17));
    }

    #[test]
    fn best_hold_decision_no_rerolls() {
        let d = dice([3, 3, 3, 3, 3]);
        let scorecard = make_scorecard();
        let decision = best_hold_decision(&d, &scorecard, RollCount::new(3).unwrap()).unwrap();
        // No re-rolls left, should hold all
        assert_eq!(decision.holds().held_count(), 5);
    }

    #[test]
    fn best_hold_decision_with_rerolls() {
        let d = dice([1, 2, 3, 5, 6]);
        let scorecard = make_scorecard();
        let decision = best_hold_decision(&d, &scorecard, RollCount::FIRST).unwrap();
        // Should hold some dice (the best strategy)
        assert!(decision.expected_value().get() > 0.0);
    }

    #[test]
    fn expected_value_for_category_no_reroll() {
        let d = dice([5, 5, 5, 5, 5]);
        let holds = HoldMask::all(); // Hold all, no re-roll
        let ev = expected_value_for_category(&d, holds, ScoreCategory::Yahtzee).unwrap();
        assert_eq!(ev.get(), 50.0);
    }

    #[test]
    fn expected_value_for_category_full_reroll() {
        let d = dice([1, 1, 1, 1, 1]);
        let holds = HoldMask::none(); // Re-roll all
        let ev = expected_value_for_category(&d, holds, ScoreCategory::Chance).unwrap();
        // Chance EV for rolling 5 dice = average sum = 5 * 3.5 = 17.5
        assert!((ev.get() - 17.5).abs() < 0.01);
    }

    #[test]
    fn improvement_probability_no_reroll() {
        let d = dice([1, 2, 3, 4, 5]);
        let holds = HoldMask::all();
        let scorecard = make_scorecard();
        let p = improvement_probability(&d, holds, &scorecard).unwrap();
        assert!(p.is_zero());
    }
}
