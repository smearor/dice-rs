use crate::error::Result;
use crate::error::YatzyError;
use crate::models::game_settings::GameSettings;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

/// Persists game settings to a JSON file on disk.
///
/// The store handles loading and saving `GameSettings` so that
/// the player setup (names, colors, mode) is restored between
/// game sessions.
pub struct SettingsStore {
    /// The path to the settings file.
    file_path: PathBuf,
}

impl SettingsStore {
    /// Create a new settings store pointing to the given file path.
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self { file_path: file_path.into() }
    }

    /// Create a settings store using the default path in the user's config directory.
    ///
    /// Uses `XDG_CONFIG_HOME/yatzy/settings.json` on Linux.
    pub fn default_path() -> Result<Self> {
        let config_dir = dirs::config_dir().ok_or(YatzyError::SettingsIoError("no config directory".to_string()))?;
        let settings_dir = config_dir.join("yatzy");
        let file_path = settings_dir.join("settings.json");
        Ok(Self { file_path })
    }

    /// Load settings from disk.
    ///
    /// Returns `Ok(None)` if the file does not exist (first run).
    pub async fn load(&self) -> Result<Option<GameSettings>> {
        if !self.file_path.exists() {
            return Ok(None);
        }
        let mut file = fs::File::open(&self.file_path).await.map_err(|e| YatzyError::SettingsIoError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| YatzyError::SettingsIoError(e.to_string()))?;
        let settings: GameSettings = serde_json::from_str(&contents).map_err(|e| YatzyError::SettingsParseError(e.to_string()))?;
        Ok(Some(settings))
    }

    /// Save settings to disk.
    ///
    /// Creates parent directories if they don't exist.
    pub async fn save(&self, settings: &GameSettings) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| YatzyError::SettingsIoError(e.to_string()))?;
        }
        let json = serde_json::to_string_pretty(settings).map_err(|e| YatzyError::SettingsParseError(e.to_string()))?;
        let mut file = fs::File::create(&self.file_path)
            .await
            .map_err(|e| YatzyError::SettingsIoError(e.to_string()))?;
        file.write_all(json.as_bytes()).await.map_err(|e| YatzyError::SettingsIoError(e.to_string()))?;
        Ok(())
    }

    /// Get the file path used by this store.
    pub fn path(&self) -> &Path {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::game_mode::GameMode;
    use crate::models::game_settings::PlayerSettingsEntry;
    use crate::models::player_color::PlayerColor;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    #[tokio::test]
    async fn save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("yatzy_test_settings.json");
        // Clean up any previous test file
        let _ = std::fs::remove_file(&path);

        let store = SettingsStore::new(&path);
        let settings = GameSettings::new(
            GameMode::MultiPlayer,
            vec![
                PlayerSettingsEntry::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human),
                PlayerSettingsEntry::new(PlayerName::new("Bob").unwrap(), PlayerColor::BLUE, PlayerType::Computer),
            ],
        );

        store.save(&settings).await.unwrap();
        let loaded = store.load().await.unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.game_mode(), GameMode::MultiPlayer);
        assert_eq!(loaded.player_count(), 2);
        assert_eq!(loaded.players()[0].name().as_str(), "Alice");

        // Clean up
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn load_nonexistent_returns_none() {
        let path = std::env::temp_dir().join("yatzy_nonexistent_settings.json");
        let _ = std::fs::remove_file(&path);
        let store = SettingsStore::new(&path);
        let loaded = store.load().await.unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn path_returns_file_path() {
        let store = SettingsStore::new("/tmp/test_settings.json");
        assert_eq!(store.path(), std::path::Path::new("/tmp/test_settings.json"));
    }
}
