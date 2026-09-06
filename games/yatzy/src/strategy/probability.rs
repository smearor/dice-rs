use crate::error::Result;
use crate::error::YatzyError;
use crate::models::dice_set::DiceSet;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Mul;

/// A probability value between 0.0 and 1.0.
///
/// This newtype enforces the invariant that a probability is always
/// in the valid range. It is used throughout the strategy module
/// for roll outcome calculations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Probability(f64);

impl Probability {
    /// Create a probability from a raw f64 value.
    /// Returns an error if the value is not in [0.0, 1.0].
    pub fn new(value: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&value) {
            return Err(YatzyError::InvalidProbability(value));
        }
        Ok(Self(value))
    }

    /// Create a probability from a fraction (numerator / denominator).
    /// Returns an error if the denominator is 0 or the fraction is > 1.
    pub fn from_fraction(numerator: u32, denominator: u32) -> Result<Self> {
        if denominator == 0 {
            return Err(YatzyError::InvalidProbability(f64::NAN));
        }
        Self::new(numerator as f64 / denominator as f64)
    }

    /// Zero probability (impossible event).
    pub const ZERO: Self = Self(0.0);

    /// One probability (certain event).
    pub const ONE: Self = Self(1.0);

    /// Get the raw f64 value.
    pub fn get(self) -> f64 {
        self.0
    }

    /// Convert to a percentage (0-100).
    pub fn as_percent(self) -> f64 {
        self.0 * 100.0
    }

    /// Returns true if this probability is zero (impossible).
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns true if this probability is one (certain).
    pub fn is_certain(self) -> bool {
        self.0 == 1.0
    }

    /// Complement probability (1 - p).
    pub fn complement(self) -> Self {
        Self(1.0 - self.0)
    }
}

impl Mul for Probability {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Display for Probability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.as_percent())
    }
}

/// The number of sides on a single die.
const DIE_SIDES: u32 = 6;

/// The total number of possible outcomes when rolling `n` dice.
///
/// Each die has 6 sides, so rolling `n` dice produces 6^n outcomes.
fn total_outcomes(dice_count: usize) -> u32 {
    DIE_SIDES.pow(dice_count as u32)
}

/// Calculate the probability of rolling a specific face value
/// at least once when re-rolling `dice_count` dice.
///
/// Example: probability of getting at least one 6 when rolling 2 dice.
pub fn probability_of_face(dice_count: usize, _face: u8) -> Result<Probability> {
    if dice_count == 0 {
        return Ok(Probability::ZERO);
    }
    // P(at least one face) = 1 - (5/6)^n
    let p_none = (5.0_f64 / 6.0_f64).powi(dice_count as i32);
    Probability::new(1.0 - p_none)
}

/// Calculate the probability of rolling at least `n` dice showing
/// the same face value when rolling `dice_count` dice.
///
/// This uses enumeration for small dice counts (1-5) which is exact.
pub fn probability_of_n_of_a_kind(dice_count: usize, n: u8) -> Result<Probability> {
    if dice_count == 0 || n as usize > dice_count {
        return Ok(Probability::ZERO);
    }
    if n == 0 {
        return Ok(Probability::ONE);
    }

    // For 5 dice, we enumerate all 6^5 = 7776 outcomes
    if dice_count <= 5 {
        let total = total_outcomes(dice_count);
        let mut favorable: u32 = 0;
        enumerate_rolls(dice_count, &mut |roll| {
            let mut counts = [0u8; 7];
            for &v in roll {
                counts[v as usize] += 1;
            }
            if counts.iter().any(|&c| c >= n) {
                favorable += 1;
            }
        });
        return Probability::from_fraction(favorable, total);
    }

    // For larger counts, use approximation
    // P(at least n of a kind) ≈ 1 - P(no n of a kind)
    // This is complex; for our use case (max 5 dice), enumeration suffices
    Probability::new(0.0)
}

/// Calculate the probability of rolling a full house
/// (exactly 3 of one value and 2 of another) when rolling `dice_count` dice.
pub fn probability_of_full_house(dice_count: usize) -> Result<Probability> {
    if dice_count < 5 {
        return Ok(Probability::ZERO);
    }
    if dice_count == 5 {
        let total = total_outcomes(5);
        let mut favorable: u32 = 0;
        enumerate_rolls(5, &mut |roll| {
            let mut counts = [0u8; 7];
            for &v in roll {
                counts[v as usize] += 1;
            }
            let has_three = counts.contains(&3);
            let has_two = counts.contains(&2);
            if has_three && has_two {
                favorable += 1;
            }
        });
        return Probability::from_fraction(favorable, total);
    }
    Ok(Probability::ZERO)
}

/// Calculate the probability of rolling a straight of at least `length`
/// when rolling `dice_count` dice.
pub fn probability_of_straight(dice_count: usize, length: usize) -> Result<Probability> {
    if dice_count < length {
        return Ok(Probability::ZERO);
    }
    if dice_count <= 5 {
        let total = total_outcomes(dice_count);
        let mut favorable: u32 = 0;
        enumerate_rolls(dice_count, &mut |roll| {
            let mut sorted = roll.to_vec();
            sorted.sort_unstable();
            let mut max_run = 1usize;
            let mut current_run = 1usize;
            for i in 1..sorted.len() {
                if sorted[i] == sorted[i - 1] {
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
            if max_run >= length {
                favorable += 1;
            }
        });
        return Probability::from_fraction(favorable, total);
    }
    Ok(Probability::ZERO)
}

/// Calculate the probability of rolling a Yatzy (all 5 dice same value)
/// when rolling `dice_count` dice.
pub fn probability_of_yatzy(dice_count: usize) -> Result<Probability> {
    probability_of_n_of_a_kind(dice_count, 5)
}

/// Enumerate all possible outcomes of rolling `n` dice and call
/// the callback for each outcome.
///
/// Each outcome is an array of face values (1-6).
fn enumerate_rolls(dice_count: usize, callback: &mut dyn FnMut(&[u8])) {
    let mut roll = vec![1u8; dice_count];
    enumerate_recursive(&mut roll, 0, callback);
}

fn enumerate_recursive(roll: &mut [u8], pos: usize, callback: &mut dyn FnMut(&[u8])) {
    if pos == roll.len() {
        callback(roll);
        return;
    }
    for face in 1..=6 {
        roll[pos] = face;
        enumerate_recursive(roll, pos + 1, callback);
    }
}

/// Calculate the probability of improving the current dice set
/// by re-rolling the non-held dice, for a specific target category.
///
/// Returns the probability that the re-rolled dice (combined with held dice)
/// will produce a positive score in the given category.
pub fn probability_of_category_score(current: &DiceSet, holds: HoldMask, _category: crate::models::category::ScoreCategory) -> Result<Probability> {
    let reroll_count = holds.reroll_count();
    if reroll_count == 0 {
        // No re-roll, current dice stay as-is
        // Probability is 1.0 if current scores > 0, else 0.0
        let score = crate::rules::scoring::calculate_score(_category, current);
        if score.get() > 0 {
            return Ok(Probability::ONE);
        }
        return Ok(Probability::ZERO);
    }

    // For exact calculation with up to 5 dice re-rolled, enumerate
    if reroll_count <= 5 {
        let total = total_outcomes(reroll_count);
        let mut favorable: u32 = 0;

        // Build the held dice values
        let held_values: Vec<u8> = DiceSlot::all()
            .filter(|slot| holds.is_held(*slot))
            .map(|slot| current.values()[slot.get() as usize])
            .collect();

        let mut reroll = vec![1u8; reroll_count];
        enumerate_reroll_outcomes(&mut reroll, 0, &held_values, _category, &mut favorable);

        return Probability::from_fraction(favorable, total);
    }

    Ok(Probability::ZERO)
}

/// Enumerate re-roll outcomes and count favorable ones.
fn enumerate_reroll_outcomes(reroll: &mut [u8], pos: usize, held_values: &[u8], category: crate::models::category::ScoreCategory, favorable: &mut u32) {
    if pos == reroll.len() {
        // Combine held + rerolled values
        let mut all_values = held_values.to_vec();
        all_values.extend_from_slice(reroll);
        let dice = DiceSet::from_values([all_values[0], all_values[1], all_values[2], all_values[3], all_values[4]]).unwrap_or_default();
        let score = crate::rules::scoring::calculate_score(category, &dice);
        if score.get() > 0 {
            *favorable += 1;
        }
        return;
    }
    for face in 1..=6 {
        reroll[pos] = face;
        enumerate_reroll_outcomes(reroll, pos + 1, held_values, category, favorable);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_new_valid() {
        assert_eq!(Probability::new(0.5).unwrap().get(), 0.5);
        assert_eq!(Probability::new(0.0).unwrap().get(), 0.0);
        assert_eq!(Probability::new(1.0).unwrap().get(), 1.0);
    }

    #[test]
    fn probability_new_invalid() {
        assert!(Probability::new(-0.1).is_err());
        assert!(Probability::new(1.1).is_err());
    }

    #[test]
    fn probability_from_fraction() {
        let p = Probability::from_fraction(1, 6).unwrap();
        assert!((p.get() - 0.16667).abs() < 0.01);
    }

    #[test]
    fn probability_from_fraction_zero_denominator() {
        assert!(Probability::from_fraction(1, 0).is_err());
    }

    #[test]
    fn probability_zero_and_one() {
        assert!(Probability::ZERO.is_zero());
        assert!(Probability::ONE.is_certain());
    }

    #[test]
    fn probability_complement() {
        let p = Probability::new(0.3).unwrap();
        assert!((p.complement().get() - 0.7).abs() < 0.001);
    }

    #[test]
    fn probability_display() {
        let p = Probability::new(0.5).unwrap();
        assert_eq!(p.to_string(), "50.0%");
    }

    #[test]
    fn probability_of_face_one_die() {
        let p = probability_of_face(1, 6).unwrap();
        assert!((p.get() - 1.0 / 6.0).abs() < 0.001);
    }

    #[test]
    fn probability_of_face_five_dice() {
        let p = probability_of_face(5, 6).unwrap();
        // P(at least one 6 in 5 dice) = 1 - (5/6)^5
        let expected = 1.0 - (5.0_f64 / 6.0_f64).powi(5);
        assert!((p.get() - expected).abs() < 0.001);
    }

    #[test]
    fn probability_of_yatzy_five_dice() {
        let p = probability_of_yatzy(5).unwrap();
        // There are 6 Yatzy outcomes out of 7776
        assert!((p.get() - 6.0 / 7776.0).abs() < 0.0001);
    }

    #[test]
    fn probability_of_full_house_five_dice() {
        let p = probability_of_full_house(5).unwrap();
        // 300 full house outcomes out of 7776
        assert!((p.get() - 300.0 / 7776.0).abs() < 0.001);
    }

    #[test]
    fn probability_of_straight_four_in_five() {
        let p = probability_of_straight(5, 4).unwrap();
        // Should be reasonably likely
        assert!(p.get() > 0.1);
    }

    #[test]
    fn probability_of_n_of_a_kind_three_in_five() {
        let p = probability_of_n_of_a_kind(5, 3).unwrap();
        // Should be around 21.3%
        assert!(p.get() > 0.15 && p.get() < 0.25);
    }

    #[test]
    fn probability_of_face_zero_dice() {
        let p = probability_of_face(0, 6).unwrap();
        assert!(p.is_zero());
    }
}
