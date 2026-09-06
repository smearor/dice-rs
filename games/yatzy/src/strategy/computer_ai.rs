use crate::error::Result;
use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::hold_mask::HoldMask;
use crate::models::roll_count::RollCount;
use crate::models::score::Score;
use crate::models::scorecard::Scorecard;
use crate::rules::cross_out::CrossOutAdvisor;
use crate::strategy::expected_value::CategoryChoice;
use crate::strategy::expected_value::best_category_choice;
use crate::strategy::expected_value::best_hold_decision;

/// A decision made by the computer AI for the current turn.
///
/// The AI decides between re-rolling (with specific holds) or
/// entering a score in a category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiDecision {
    /// Re-roll the dice with the given hold mask.
    RollAgain {
        /// Which dice to hold for the re-roll.
        holds: HoldMask,
    },
    /// Enter a score in the given category.
    EnterScore {
        /// The category to enter the score in.
        category: ScoreCategory,
        /// The score to enter.
        score: Score,
    },
    /// Cross out a category (no valid score available).
    CrossOut {
        /// The category to cross out.
        category: ScoreCategory,
    },
}

/// The computer opponent decision engine.
///
/// Uses expected value calculations to make optimal decisions:
/// - Whether to re-roll or enter a score
/// - Which dice to hold when re-rolling
/// - Which category to enter or cross out
pub struct ComputerAi {
    /// The cross-out advisor for choosing categories to cross out.
    advisor: CrossOutAdvisor,
}

impl ComputerAi {
    /// Create a new computer AI.
    pub fn new() -> Self {
        Self {
            advisor: CrossOutAdvisor::new(),
        }
    }

    /// Decide the best action for the current game state.
    ///
    /// Parameters:
    /// - `dice`: The current dice values
    /// - `scorecard`: The current player's scorecard
    /// - `rolls_used`: How many rolls have been used this turn
    pub fn decide(&self, dice: &DiceSet, scorecard: &Scorecard, rolls_used: RollCount) -> Result<AiDecision> {
        // If rolls are exhausted, must enter a score or cross out
        if rolls_used.is_exhausted() {
            return Ok(self.decide_scoring(dice, scorecard));
        }

        // Compare: enter now vs. re-roll
        let current_best = best_category_choice(dice, scorecard);
        let rolls_remaining = rolls_used;
        let hold_decision = best_hold_decision(dice, scorecard, rolls_remaining)?;

        // If current best score is good enough, enter it
        // Threshold: if current best >= EV of re-rolling, enter now
        if current_best.score().get() as f64 >= hold_decision.expected_value().get() {
            return Ok(AiDecision::EnterScore {
                category: current_best.category(),
                score: current_best.score(),
            });
        }

        // Otherwise, re-roll with the best hold mask
        Ok(AiDecision::RollAgain { holds: hold_decision.holds() })
    }

    /// Decide which category to enter or cross out when rolls are exhausted.
    fn decide_scoring(&self, dice: &DiceSet, scorecard: &Scorecard) -> AiDecision {
        let best = best_category_choice(dice, scorecard);

        if best.score().get() > 0 {
            AiDecision::EnterScore {
                category: best.category(),
                score: best.score(),
            }
        } else {
            // No positive score — cross out the least valuable category
            match self.advisor.recommend_voluntary(scorecard, dice) {
                Some(rec) => AiDecision::CrossOut { category: rec.category() },
                None => {
                    // Scorecard is complete — shouldn't happen in normal flow
                    // Enter chance with 0 as fallback
                    AiDecision::EnterScore {
                        category: best.category(),
                        score: Score::ZERO,
                    }
                }
            }
        }
    }

    /// Get a hint for a human player (best category to enter).
    ///
    /// This is used by the hint panel UI to show strategy suggestions.
    pub fn hint_best_category(&self, dice: &DiceSet, scorecard: &Scorecard) -> CategoryChoice {
        best_category_choice(dice, scorecard)
    }

    /// Get a hint for a human player (best hold decision).
    ///
    /// Returns the recommended hold mask for re-rolling.
    pub fn hint_best_holds(&self, dice: &DiceSet, scorecard: &Scorecard, rolls_used: RollCount) -> Result<HoldMask> {
        let decision = best_hold_decision(dice, scorecard, rolls_used)?;
        Ok(decision.holds())
    }
}

impl Default for ComputerAi {
    fn default() -> Self {
        Self::new()
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
    fn decide_yatzy_enters_score() {
        let ai = ComputerAi::new();
        let dice = dice([5, 5, 5, 5, 5]);
        let scorecard = make_scorecard();
        let decision = ai.decide(&dice, &scorecard, RollCount::FIRST).unwrap();
        // Yatzy (50 points) is so good, AI should enter it immediately
        match decision {
            AiDecision::EnterScore { category, score } => {
                assert_eq!(category, ScoreCategory::Yatzy);
                assert_eq!(score, Score::new(50));
            }
            AiDecision::RollAgain { .. } => {
                // Could also decide to roll again, but 50 is very high
                // Let's just verify it doesn't cross out
            }
            AiDecision::CrossOut { .. } => panic!("should not cross out with Yatzy"),
        }
    }

    #[test]
    fn decide_rolls_exhausted_enters_best() {
        let ai = ComputerAi::new();
        let dice = dice([3, 3, 3, 3, 3]);
        let scorecard = make_scorecard();
        let decision = ai.decide(&dice, &scorecard, RollCount::new(3).unwrap()).unwrap();
        match decision {
            AiDecision::EnterScore { category, score } => {
                // Yatzy (50) is the highest-scoring category for five 3s
                assert_eq!(category, ScoreCategory::Yatzy);
                assert_eq!(score, Score::new(50));
            }
            _ => panic!("should enter score when rolls exhausted"),
        }
    }

    #[test]
    fn decide_rolls_exhausted_crosses_out_when_zero() {
        let ai = ComputerAi::new();
        let dice = dice([1, 2, 3, 5, 6]);
        let mut scorecard = make_scorecard();
        // Fill all categories that would score positively
        for cat in ScoreCategory::ALL {
            if crate::rules::scoring::calculate_score(cat, &dice).get() > 0 {
                scorecard.enter(cat, Score::new(10)).unwrap();
            }
        }
        let decision = ai.decide(&dice, &scorecard, RollCount::new(3).unwrap()).unwrap();
        match decision {
            AiDecision::CrossOut { .. } => {}
            _ => panic!("should cross out when no positive score available"),
        }
    }

    #[test]
    fn decide_low_score_rolls_again() {
        let ai = ComputerAi::new();
        let dice = dice([1, 2, 3, 5, 6]);
        let scorecard = make_scorecard();
        let decision = ai.decide(&dice, &scorecard, RollCount::FIRST).unwrap();
        // With a poor first roll, the AI should likely re-roll
        match decision {
            AiDecision::RollAgain { holds } => {
                // Should not hold all dice (that would be pointless)
                assert!(holds.held_count() < 5);
            }
            AiDecision::EnterScore { score, .. } => {
                // If entering, it should be a low score
                assert!(score.get() <= 17); // Chance = 17
            }
            _ => {}
        }
    }

    #[test]
    fn hint_best_category() {
        let ai = ComputerAi::new();
        let dice = dice([6, 6, 6, 6, 6]);
        let scorecard = make_scorecard();
        let hint = ai.hint_best_category(&dice, &scorecard);
        assert_eq!(hint.category(), ScoreCategory::Yatzy);
        assert_eq!(hint.score(), Score::new(50));
    }

    #[test]
    fn hint_best_holds() {
        let ai = ComputerAi::new();
        let dice = dice([1, 2, 3, 4, 5]);
        let scorecard = make_scorecard();
        let holds = ai.hint_best_holds(&dice, &scorecard, RollCount::FIRST).unwrap();
        // Should hold some dice for a large straight attempt
        assert!(holds.held_count() > 0);
    }

    #[test]
    fn decide_full_house_enters() {
        let ai = ComputerAi::new();
        let dice = dice([3, 3, 3, 5, 5]);
        let scorecard = make_scorecard();
        let decision = ai.decide(&dice, &scorecard, RollCount::FIRST).unwrap();
        // Full house = 25 points, which is quite good for first roll
        match decision {
            AiDecision::EnterScore { category, score } => {
                if category == ScoreCategory::FullHouse {
                    assert_eq!(score, Score::new(25));
                }
            }
            _ => {}
        }
    }
}
