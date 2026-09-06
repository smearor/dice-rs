use crate::models::category::ScoreCategory;
use crate::models::hold_mask::HoldMask;
use crate::models::score::Score;
use serde::Deserialize;
use serde::Serialize;

/// A semantic action that a player can take during their turn.
///
/// This enum encapsulates all possible player actions in a type-safe
/// manner, avoiding stringly-typed APIs. The game controller validates
/// each action against the current game state before executing it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnAction {
    /// Roll the non-held dice.
    Roll,
    /// Update the hold mask for the current dice.
    Hold {
        /// The new hold mask to apply.
        holds: HoldMask,
    },
    /// Enter a score in the given category.
    EnterScore {
        /// The category to enter the score in.
        category: ScoreCategory,
        /// The score to enter (calculated from dice values).
        score: Score,
    },
    /// Cross out a category (score zero).
    CrossOut {
        /// The category to cross out.
        category: ScoreCategory,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_action_equality() {
        assert_eq!(TurnAction::Roll, TurnAction::Roll);
    }

    #[test]
    fn hold_action_equality() {
        let holds = HoldMask::none();
        assert_eq!(TurnAction::Hold { holds }, TurnAction::Hold { holds });
    }

    #[test]
    fn enter_score_action_equality() {
        assert_eq!(
            TurnAction::EnterScore {
                category: ScoreCategory::Ones,
                score: Score::new(3)
            },
            TurnAction::EnterScore {
                category: ScoreCategory::Ones,
                score: Score::new(3)
            }
        );
    }

    #[test]
    fn cross_out_action_equality() {
        assert_eq!(
            TurnAction::CrossOut {
                category: ScoreCategory::Yatzy
            },
            TurnAction::CrossOut {
                category: ScoreCategory::Yatzy
            }
        );
    }

    #[test]
    fn actions_are_distinct() {
        assert_ne!(TurnAction::Roll, TurnAction::Hold { holds: HoldMask::none() });
    }
}
