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

impl std::fmt::Display for TurnPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AwaitingRoll => write!(f, "Warte auf Wurf"),
            Self::Rolling => write!(f, "Würfeln..."),
            Self::Holding => write!(f, "Würfel halten"),
            Self::Scoring => write!(f, "Punkte eintragen"),
            Self::TurnEnd => write!(f, "Zug beendet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_german() {
        assert_eq!(TurnPhase::AwaitingRoll.to_string(), "Warte auf Wurf");
        assert_eq!(TurnPhase::Rolling.to_string(), "Würfeln...");
        assert_eq!(TurnPhase::Holding.to_string(), "Würfel halten");
        assert_eq!(TurnPhase::Scoring.to_string(), "Punkte eintragen");
        assert_eq!(TurnPhase::TurnEnd.to_string(), "Zug beendet");
    }
}
