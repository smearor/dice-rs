use crate::i18n::Localized;
use crate::impl_display_localized;
use serde::Deserialize;
use serde::Serialize;

/// The phase of a player's turn in the game state machine.
///
/// The turn progresses through these phases:
/// 1. `AwaitingRoll` — waiting for the player to roll the dice
/// 2. `Rolling` — dice are physically rolling
/// 3. `Holding` — dice are stable, player decides holds or re-rolls
/// 4. `Scoring` — player must select a category to enter/cross out
/// 5. `TurnEnd` — score has been entered, transitioning to next player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnPhase {
    /// Waiting for the player to start rolling.
    AwaitingRoll,
    /// Dice are physically rolling (RollStart received, no Stable yet).
    Rolling,
    /// Dice are stable. Player can hold dice and roll again, or proceed to scoring.
    Holding,
    /// Player must select a category to enter or cross out.
    Scoring,
    /// Turn has ended. Transitioning to the next player.
    TurnEnd,
}

impl Localized for TurnPhase {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::AwaitingRoll => "turn-phase-awaiting-roll",
            Self::Rolling => "turn-phase-rolling",
            Self::Holding => "turn-phase-holding",
            Self::Scoring => "turn-phase-scoring",
            Self::TurnEnd => "turn-phase-turn-end",
        }
    }
}

impl_display_localized!(TurnPhase);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_matches_localized() {
        assert_eq!(TurnPhase::AwaitingRoll.to_string(), TurnPhase::AwaitingRoll.localized());
        assert_eq!(TurnPhase::Rolling.to_string(), TurnPhase::Rolling.localized());
        assert_eq!(TurnPhase::Holding.to_string(), TurnPhase::Holding.localized());
        assert_eq!(TurnPhase::Scoring.to_string(), TurnPhase::Scoring.localized());
        assert_eq!(TurnPhase::TurnEnd.to_string(), TurnPhase::TurnEnd.localized());
    }
}
