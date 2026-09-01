use crate::models::game_mode::GameMode;
use crate::models::player_color::PlayerColor;
use crate::models::player_name::PlayerName;
use crate::models::player_type::PlayerType;
use serde::Deserialize;
use serde::Serialize;

/// A single player's persisted settings.
///
/// Stored as part of `GameSettings` when the game configuration is saved.
/// Contains the player's name, color, and type (human or computer).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSettingsEntry {
    /// The player's display name.
    name: PlayerName,
    /// The player's color for LED and UI identification.
    color: PlayerColor,
    /// Whether this player is human or the computer opponent.
    player_type: PlayerType,
}

impl PlayerSettingsEntry {
    /// Create a new player settings entry.
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
}

/// Persisted game settings containing the full player configuration.
///
/// This struct is serialized to JSON and stored on disk so that
/// the player setup is restored between game sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSettings {
    /// The game mode (single-player or multi-player).
    game_mode: GameMode,
    /// The list of player configurations.
    players: Vec<PlayerSettingsEntry>,
}

impl GameSettings {
    /// Create a new game settings with the given mode and players.
    pub fn new(game_mode: GameMode, players: Vec<PlayerSettingsEntry>) -> Self {
        Self { game_mode, players }
    }

    /// Get the game mode.
    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    /// Get the player settings entries.
    pub fn players(&self) -> &[PlayerSettingsEntry] {
        &self.players
    }

    /// Get the number of players.
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Create default settings for a single-player game.
    pub fn default_single_player() -> Self {
        let name = PlayerName::new("Spieler 1").unwrap_or_else(|_| PlayerName::new("Player").unwrap_or_else(|_| PlayerName::new("Spieler").unwrap()));
        Self {
            game_mode: GameMode::SinglePlayer,
            players: vec![PlayerSettingsEntry::new(name, PlayerColor::RED, PlayerType::Human)],
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self::default_single_player()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_settings() {
        let settings = GameSettings::new(
            GameMode::MultiPlayer,
            vec![
                PlayerSettingsEntry::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human),
                PlayerSettingsEntry::new(PlayerName::new("Bob").unwrap(), PlayerColor::BLUE, PlayerType::Human),
            ],
        );
        assert_eq!(settings.game_mode(), GameMode::MultiPlayer);
        assert_eq!(settings.player_count(), 2);
    }

    #[test]
    fn default_single_player() {
        let settings = GameSettings::default_single_player();
        assert_eq!(settings.game_mode(), GameMode::SinglePlayer);
        assert_eq!(settings.player_count(), 1);
    }

    #[test]
    fn default_is_single_player() {
        let settings = GameSettings::default();
        assert_eq!(settings.game_mode(), GameMode::SinglePlayer);
    }

    #[test]
    fn player_settings_entry_accessors() {
        let entry = PlayerSettingsEntry::new(PlayerName::new("Carol").unwrap(), PlayerColor::GREEN, PlayerType::Computer);
        assert_eq!(entry.name().as_str(), "Carol");
        assert_eq!(entry.color(), PlayerColor::GREEN);
        assert_eq!(entry.player_type(), PlayerType::Computer);
    }

    #[test]
    fn serialize_deserialize() {
        let settings = GameSettings::new(
            GameMode::MultiPlayer,
            vec![PlayerSettingsEntry::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human)],
        );
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: GameSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }
}
