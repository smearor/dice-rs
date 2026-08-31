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

impl std::fmt::Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "Mensch"),
            Self::Computer => write!(f, "Computer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_german() {
        assert_eq!(PlayerType::Human.to_string(), "Mensch");
        assert_eq!(PlayerType::Computer.to_string(), "Computer");
    }
}
