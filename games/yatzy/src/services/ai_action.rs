use crate::models::category::ScoreCategory;
use crate::models::hold_mask::HoldMask;
use crate::models::score::Score;

/// An action the computer AI wants to take, translated into controller-level terms.
///
/// The UI layer is responsible for executing this action via `GameController::execute`
/// and updating the visual state accordingly. A small delay before execution
/// gives the human player time to see the AI's decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiAction {
    /// Roll the dice (first roll of a turn).
    Roll,
    /// Set the given holds and then re-roll.
    RollWithHolds {
        /// The hold mask to apply before rolling.
        holds: HoldMask,
    },
    /// Enter a score in the given category.
    EnterScore {
        /// The category to enter the score in.
        category: ScoreCategory,
        /// The score to enter.
        score: Score,
    },
    /// Cross out a category.
    CrossOut {
        /// The category to cross out.
        category: ScoreCategory,
    },
}
