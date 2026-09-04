use crate::fl_write;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The game mode determining player interaction style.
///
/// In `SinglePlayer` mode, strategy hints are shown to help the player
/// learn optimal play. In `MultiPlayer` mode, hints are disabled to
/// ensure fair play between human players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameMode {
    /// Single-player mode with AI opponent and strategy hints enabled.
    SinglePlayer,
    /// Multi-player pass-and-play mode with hints disabled for fairness.
    MultiPlayer,
}

impl GameMode {
    /// Returns true if strategy hints should be shown in this mode.
    pub fn hints_enabled(self) -> bool {
        match self {
            Self::SinglePlayer => true,
            Self::MultiPlayer => false,
        }
    }

    /// Returns true if this is a pass-and-play mode requiring turn transitions.
    pub fn requires_turn_transitions(self) -> bool {
        match self {
            Self::SinglePlayer => false,
            Self::MultiPlayer => true,
        }
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SinglePlayer => fl_write!(f, "game-mode-single-player"),
            Self::MultiPlayer => fl_write!(f, "game-mode-multi-player"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hints_enabled_single_player() {
        assert!(GameMode::SinglePlayer.hints_enabled());
    }

    #[test]
    fn hints_disabled_multi_player() {
        assert!(!GameMode::MultiPlayer.hints_enabled());
    }

    #[test]
    fn turn_transitions_only_multi_player() {
        assert!(!GameMode::SinglePlayer.requires_turn_transitions());
        assert!(GameMode::MultiPlayer.requires_turn_transitions());
    }

    #[test]
    fn display_single_player() {
        assert_eq!(GameMode::SinglePlayer.to_string(), "Einzelspieler");
    }

    #[test]
    fn display_multi_player() {
        assert_eq!(GameMode::MultiPlayer.to_string(), "Mehrspieler");
    }

    #[test]
    fn modes_are_distinct() {
        assert_ne!(GameMode::SinglePlayer, GameMode::MultiPlayer);
    }
}
