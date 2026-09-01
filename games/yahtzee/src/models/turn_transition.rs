use crate::models::player::Player;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::models::round_number::RoundNumber;
use serde::Deserialize;
use serde::Serialize;

/// Data for a pass-and-play turn transition screen.
///
/// When the active player changes in multi-player mode, this struct
/// carries the information needed to display the transition overlay:
/// which player is next, their color for LED indication, and the
/// current round number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnTransition {
    /// The index of the next player.
    player_index: PlayerIndex,
    /// The next player's display name.
    player_name: String,
    /// The next player's color for LED and UI indication.
    player_color: PlayerColor,
    /// The current round number.
    round: RoundNumber,
}

impl TurnTransition {
    /// Create a new turn transition from the next player and round.
    pub fn new(player: &Player, player_index: PlayerIndex, round: RoundNumber) -> Self {
        Self {
            player_index,
            player_name: player.name().as_str().to_string(),
            player_color: player.color(),
            round,
        }
    }

    /// Get the next player's index.
    pub fn player_index(&self) -> PlayerIndex {
        self.player_index
    }

    /// Get the next player's name.
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    /// Get the next player's color.
    pub fn player_color(&self) -> PlayerColor {
        self.player_color
    }

    /// Get the current round number.
    pub fn round(&self) -> RoundNumber {
        self.round
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    fn make_player(name: &str, color: PlayerColor) -> Player {
        Player::new(PlayerName::new(name).unwrap(), color, PlayerType::Human)
    }

    #[test]
    fn new_creates_transition() {
        let player = make_player("Alice", PlayerColor::RED);
        let transition = TurnTransition::new(&player, PlayerIndex::new(0), RoundNumber::FIRST);
        assert_eq!(transition.player_index(), PlayerIndex::new(0));
        assert_eq!(transition.player_name(), "Alice");
        assert_eq!(transition.player_color(), PlayerColor::RED);
        assert_eq!(transition.round(), RoundNumber::FIRST);
    }

    #[test]
    fn player_name_returns_str() {
        let player = make_player("Bob", PlayerColor::BLUE);
        let transition = TurnTransition::new(&player, PlayerIndex::new(1), RoundNumber::FIRST);
        assert_eq!(transition.player_name(), "Bob");
    }
}
