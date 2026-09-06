use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::game_status::GameStatus;
use crate::models::score::Score;
use crate::models::standing::StandingEntry;
use crate::models::turn_phase::TurnPhase;
use crate::models::turn_transition::TurnTransition;
use crate::rules::cross_out::CrossOutRecommendation;
use crate::services::led_effect::LedEffect;

/// Events emitted by the `GameController` for the UI to react to.
///
/// These events represent state changes that the UI needs to visualize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerEvent {
    /// The game status changed (e.g. Playing → GameOver).
    StatusChanged {
        /// The new game status.
        status: GameStatus,
    },
    /// A new player's turn started.
    TurnStarted {
        /// The index of the player whose turn it is.
        player_index: usize,
    },
    /// The turn phase changed.
    PhaseChanged {
        /// The new turn phase.
        phase: TurnPhase,
    },
    /// A roll was completed and dice values are available.
    RollCompleted {
        /// The final dice values.
        dice: DiceSet,
    },
    /// A score was entered for a player.
    ScoreEntered {
        /// The player index.
        player_index: usize,
        /// The category that was scored.
        category: ScoreCategory,
        /// The score that was entered.
        score: Score,
    },
    /// A category was crossed out for a player.
    CategoryCrossedOut {
        /// The player index.
        player_index: usize,
        /// The category that was crossed out.
        category: ScoreCategory,
    },
    /// The game is over and standings are available.
    GameOver {
        /// The final standings, ranked by score.
        standings: Vec<StandingEntry>,
    },
    /// A cross-out recommendation was generated.
    CrossOutRecommended {
        /// The recommended category to cross out.
        recommendation: CrossOutRecommendation,
    },
    /// A turn transition occurred (pass-and-play in multi-player mode).
    TurnTransition {
        /// The transition data for the next player's turn.
        transition: TurnTransition,
    },
    /// A celebration effect should be applied for a special roll.
    Celebration {
        /// The LED effect to apply.
        effect: LedEffect,
    },
}
