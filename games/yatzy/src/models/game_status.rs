use crate::i18n::Localized;
use crate::impl_display_localized;
use serde::Deserialize;
use serde::Serialize;

/// The overall status of a Yatzy game.
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

impl Localized for GameStatus {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::Setup => "game-status-setup",
            Self::Playing => "game-status-playing",
            Self::GameOver => "game-status-game-over",
        }
    }
}

impl_display_localized!(GameStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_setup() {
        assert_eq!(GameStatus::Setup.to_string(), GameStatus::Setup.localized());
    }

    #[test]
    fn display_playing() {
        assert_eq!(GameStatus::Playing.to_string(), GameStatus::Playing.localized());
    }

    #[test]
    fn display_game_over() {
        assert_eq!(GameStatus::GameOver.to_string(), GameStatus::GameOver.localized());
    }

    #[test]
    fn statuses_are_distinct() {
        assert_ne!(GameStatus::Setup, GameStatus::Playing);
        assert_ne!(GameStatus::Playing, GameStatus::GameOver);
        assert_ne!(GameStatus::Setup, GameStatus::GameOver);
    }
}
