use crate::error::Result;
use crate::error::YahtzeeError;
use crate::models::player::Player;
use crate::models::player_color::PlayerColor;
use crate::models::player_name::PlayerName;
use crate::models::player_type::PlayerType;
use serde::Deserialize;
use serde::Serialize;

/// Configuration for a single player during game setup.
///
/// This struct captures the user's choices in the player setup screen
/// before the game starts. It is converted to a `Player` when the game begins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerConfig {
    /// The player's display name.
    name: PlayerName,
    /// The player's color for LED and UI identification.
    color: PlayerColor,
    /// Whether this player is human or the computer opponent.
    player_type: PlayerType,
}

impl PlayerConfig {
    /// Create a new player configuration.
    pub fn new(name: PlayerName, color: PlayerColor, player_type: PlayerType) -> Self {
        Self { name, color, player_type }
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

    /// Convert this configuration into a `Player` with an empty scorecard.
    pub fn into_player(self) -> Player {
        Player::new(self.name, self.color, self.player_type)
    }
}

/// A collection of player configurations validated for a game setup.
///
/// Ensures that all player names and colors are unique before the game starts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSetup {
    /// The list of player configurations.
    configs: Vec<PlayerConfig>,
}

impl PlayerSetup {
    /// Create a new player setup from a list of configurations.
    ///
    /// Returns an error if:
    /// - The list is empty
    /// - There are more than 6 players
    /// - Two players have the same name
    /// - Two players have the same color
    pub fn new(configs: Vec<PlayerConfig>) -> Result<Self> {
        if configs.is_empty() {
            return Err(YahtzeeError::NotEnoughPlayers(0));
        }
        if configs.len() > PlayerColor::DEFAULTS.len() {
            return Err(YahtzeeError::TooManyPlayers(configs.len()));
        }

        // Check for duplicate names
        for i in 0..configs.len() {
            for j in (i + 1)..configs.len() {
                if configs[i].name == configs[j].name {
                    return Err(YahtzeeError::DuplicateName(configs[i].name.as_str().to_string()));
                }
                if configs[i].color == configs[j].color {
                    return Err(YahtzeeError::DuplicateColor(configs[i].color.to_string()));
                }
            }
        }

        Ok(Self { configs })
    }

    /// Get the list of player configurations.
    pub fn configs(&self) -> &[PlayerConfig] {
        &self.configs
    }

    /// Get the number of players.
    pub fn count(&self) -> usize {
        self.configs.len()
    }

    /// Convert all configurations into players.
    pub fn into_players(self) -> Vec<Player> {
        self.configs.into_iter().map(PlayerConfig::into_player).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(name: &str, color: PlayerColor) -> PlayerConfig {
        PlayerConfig::new(PlayerName::new(name).unwrap(), color, PlayerType::Human)
    }

    #[test]
    fn player_config_new() {
        let cfg = config("Alice", PlayerColor::RED);
        assert_eq!(cfg.name().as_str(), "Alice");
        assert_eq!(cfg.color(), PlayerColor::RED);
        assert_eq!(cfg.player_type(), PlayerType::Human);
    }

    #[test]
    fn player_config_into_player() {
        let cfg = config("Bob", PlayerColor::BLUE);
        let player = cfg.into_player();
        assert_eq!(player.name().as_str(), "Bob");
        assert_eq!(player.color(), PlayerColor::BLUE);
    }

    #[test]
    fn player_setup_valid() {
        let setup = PlayerSetup::new(vec![config("Alice", PlayerColor::RED), config("Bob", PlayerColor::BLUE)]).unwrap();
        assert_eq!(setup.count(), 2);
    }

    #[test]
    fn player_setup_empty_fails() {
        assert!(PlayerSetup::new(vec![]).is_err());
    }

    #[test]
    fn player_setup_too_many_fails() {
        let configs: Vec<PlayerConfig> = PlayerColor::DEFAULTS
            .iter()
            .enumerate()
            .map(|(i, color)| config(&format!("Player{i}"), *color))
            .collect();
        let mut configs = configs;
        configs.push(config("Extra", PlayerColor::WHITE));
        assert!(PlayerSetup::new(configs).is_err());
    }

    #[test]
    fn player_setup_duplicate_name_fails() {
        let result = PlayerSetup::new(vec![config("Alice", PlayerColor::RED), config("Alice", PlayerColor::BLUE)]);
        assert!(result.is_err());
    }

    #[test]
    fn player_setup_duplicate_color_fails() {
        let result = PlayerSetup::new(vec![config("Alice", PlayerColor::RED), config("Bob", PlayerColor::RED)]);
        assert!(result.is_err());
    }

    #[test]
    fn player_setup_into_players() {
        let setup = PlayerSetup::new(vec![config("Alice", PlayerColor::RED), config("Bob", PlayerColor::BLUE)]).unwrap();
        let players = setup.into_players();
        assert_eq!(players.len(), 2);
        assert_eq!(players[0].name().as_str(), "Alice");
        assert_eq!(players[1].name().as_str(), "Bob");
    }
}
