use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The overall status of a Yahtzee game.
///
/// This is distinct from `TurnPhase` (which tracks the micro-state
/// within a single player's turn). `GameStatus` tracks the macro-level
/// game lifecycle: setup, active play, or game over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameStatus {
    /// Game is being set up (players joining, dice connecting).
    Setup,
    /// Game is actively in progress.
    Playing,
    /// Game has ended — all rounds completed, winners determined.
    GameOver,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Setup => write!(f, "Setup"),
            Self::Playing => write!(f, "Spiel läuft"),
            Self::GameOver => write!(f, "Spiel beendet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_setup() {
        assert_eq!(GameStatus::Setup.to_string(), "Setup");
    }

    #[test]
    fn display_playing() {
        assert_eq!(GameStatus::Playing.to_string(), "Spiel läuft");
    }

    #[test]
    fn display_game_over() {
        assert_eq!(GameStatus::GameOver.to_string(), "Spiel beendet");
    }

    #[test]
    fn statuses_are_distinct() {
        assert_ne!(GameStatus::Setup, GameStatus::Playing);
        assert_ne!(GameStatus::Playing, GameStatus::GameOver);
        assert_ne!(GameStatus::Setup, GameStatus::GameOver);
    }
}
