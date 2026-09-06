use crate::i18n::Localized;
use crate::impl_display_localized;
use serde::Deserialize;
use serde::Serialize;

/// Whether a player is a human or the computer opponent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerType {
    /// A human player who physically rolls the dice.
    Human,
    /// The computer opponent. The human player rolls on its behalf.
    Computer,
}

impl Localized for PlayerType {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::Human => "player-type-human",
            Self::Computer => "player-type-computer",
        }
    }
}

impl_display_localized!(PlayerType);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_matches_localized() {
        assert_eq!(PlayerType::Human.to_string(), PlayerType::Human.localized());
        assert_eq!(PlayerType::Computer.to_string(), PlayerType::Computer.localized());
    }
}
