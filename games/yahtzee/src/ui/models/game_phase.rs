use crate::i18n::Localized;
use crate::impl_display_localized;

/// Top-level UI phase, controlling which screen is shown.
///
/// This is distinct from `TurnPhase` (which tracks the micro-state
/// within a single turn). `GamePhase` determines the overall layout
/// of the main window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamePhase {
    /// Dice scanning and connection setup screen.
    Setup,
    /// Active gameplay screen.
    Playing,
    /// Game over screen with final scores.
    GameOver,
}

impl Localized for GamePhase {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::Setup => "game-phase-setup",
            Self::Playing => "game-phase-playing",
            Self::GameOver => "game-phase-game-over",
        }
    }
}

impl_display_localized!(GamePhase);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_setup() {
        assert_eq!(GamePhase::Setup.to_string(), GamePhase::Setup.localized());
    }

    #[test]
    fn display_playing() {
        assert_eq!(GamePhase::Playing.to_string(), GamePhase::Playing.localized());
    }

    #[test]
    fn display_game_over() {
        assert_eq!(GamePhase::GameOver.to_string(), GamePhase::GameOver.localized());
    }

    #[test]
    fn phases_are_distinct() {
        assert_ne!(GamePhase::Setup, GamePhase::Playing);
        assert_ne!(GamePhase::Playing, GamePhase::GameOver);
        assert_ne!(GamePhase::Setup, GamePhase::GameOver);
    }
}
