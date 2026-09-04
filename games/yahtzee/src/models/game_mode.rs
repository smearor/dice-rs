use crate::i18n::Localized;
use crate::impl_display_localized;
use serde::Deserialize;
use serde::Serialize;

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

impl Localized for GameMode {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::SinglePlayer => "game-mode-single-player",
            Self::MultiPlayer => "game-mode-multi-player",
        }
    }
}

impl_display_localized!(GameMode);

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
        assert_eq!(GameMode::SinglePlayer.to_string(), GameMode::SinglePlayer.localized());
    }

    #[test]
    fn display_multi_player() {
        assert_eq!(GameMode::MultiPlayer.to_string(), GameMode::MultiPlayer.localized());
    }

    #[test]
    fn modes_are_distinct() {
        assert_ne!(GameMode::SinglePlayer, GameMode::MultiPlayer);
    }
}
