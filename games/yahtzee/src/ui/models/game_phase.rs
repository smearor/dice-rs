use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

impl Display for GamePhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Setup => write!(f, "Setup"),
            Self::Playing => write!(f, "Playing"),
            Self::GameOver => write!(f, "Game Over"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_setup() {
        assert_eq!(GamePhase::Setup.to_string(), "Setup");
    }

    #[test]
    fn display_playing() {
        assert_eq!(GamePhase::Playing.to_string(), "Playing");
    }

    #[test]
    fn display_game_over() {
        assert_eq!(GamePhase::GameOver.to_string(), "Game Over");
    }

    #[test]
    fn phases_are_distinct() {
        assert_ne!(GamePhase::Setup, GamePhase::Playing);
        assert_ne!(GamePhase::Playing, GamePhase::GameOver);
        assert_ne!(GamePhase::Setup, GamePhase::GameOver);
    }
}
