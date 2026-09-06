use crate::models::player_color::PlayerColor;
use crate::models::player_name::PlayerName;
use crate::models::player_type::PlayerType;
use crate::models::score::Score;
use crate::models::scorecard::Scorecard;
use serde::Deserialize;
use serde::Serialize;

/// A participant in a Yatzy game.
///
/// Each player has a name, a color for LED/UI identification, a type
/// (human or computer), and a scorecard tracking their progress through
/// the 13 categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    /// The player's display name.
    name: PlayerName,
    /// The player's color for LED indication and UI display.
    color: PlayerColor,
    /// Whether this player is human or the computer opponent.
    player_type: PlayerType,
    /// The player's scorecard.
    scorecard: Scorecard,
}

impl Player {
    /// Create a new player with an empty scorecard.
    pub fn new(name: PlayerName, color: PlayerColor, player_type: PlayerType) -> Self {
        Self {
            name,
            color,
            player_type,
            scorecard: Scorecard::new(),
        }
    }

    /// Get the player's name.
    pub fn name(&self) -> &PlayerName {
        &self.name
    }

    /// Get the player's color.
    pub fn color(&self) -> PlayerColor {
        self.color
    }

    /// Get the player's type.
    pub fn player_type(&self) -> PlayerType {
        self.player_type
    }

    /// Get the player's scorecard.
    pub fn scorecard(&self) -> &Scorecard {
        &self.scorecard
    }

    /// Get a mutable reference to the player's scorecard.
    pub fn scorecard_mut(&mut self) -> &mut Scorecard {
        &mut self.scorecard
    }

    /// Get the player's grand total score.
    pub fn grand_total(&self) -> Score {
        self.scorecard.grand_total()
    }

    /// Returns true if this player is the computer opponent.
    pub fn is_computer(&self) -> bool {
        self.player_type == PlayerType::Computer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::category::ScoreCategory;

    fn make_player() -> Player {
        Player::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human)
    }

    #[test]
    fn new_player_has_empty_scorecard() {
        let p = make_player();
        assert!(!p.scorecard().is_complete());
        assert_eq!(p.grand_total(), Score::ZERO);
    }

    #[test]
    fn is_computer() {
        let human = make_player();
        assert!(!human.is_computer());

        let computer = Player::new(PlayerName::new("CPU").unwrap(), PlayerColor::BLUE, PlayerType::Computer);
        assert!(computer.is_computer());
    }

    #[test]
    fn scorecard_mut_allows_entry() {
        let mut p = make_player();
        p.scorecard_mut().enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        assert_eq!(p.grand_total(), Score::new(3));
    }
}
