use crate::error::Result;
use crate::error::YahtzeeError;
use crate::models::highscore::HighscoreList;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

/// Persists highscore data to a JSON file on disk.
///
/// The store handles loading and saving `HighscoreList` so that
/// highscores are preserved between game sessions. Uses the user's
/// data directory (XDG_DATA_HOME on Linux) for runtime data.
pub struct HighscoreStore {
    /// The path to the highscore file.
    file_path: PathBuf,
}

impl HighscoreStore {
    /// Create a new highscore store pointing to the given file path.
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self { file_path: file_path.into() }
    }

    /// Create a highscore store using the default path in the user's data directory.
    ///
    /// Uses `XDG_DATA_HOME/yahtzee/highscore.json` on Linux.
    pub fn default_path() -> Result<Self> {
        let data_dir = dirs::data_dir().ok_or(YahtzeeError::SettingsIoError("no data directory".to_string()))?;
        let highscore_dir = data_dir.join("yahtzee");
        let file_path = highscore_dir.join("highscore.json");
        Ok(Self { file_path })
    }

    /// Load highscores from disk.
    ///
    /// Returns `Ok(None)` if the file does not exist (first run).
    pub async fn load(&self) -> Result<Option<HighscoreList>> {
        if !self.file_path.exists() {
            return Ok(None);
        }
        let mut file = fs::File::open(&self.file_path)
            .await
            .map_err(|e| YahtzeeError::SettingsIoError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|e| YahtzeeError::SettingsIoError(e.to_string()))?;
        let list: HighscoreList = serde_json::from_str(&contents).map_err(|e| YahtzeeError::SettingsParseError(e.to_string()))?;
        Ok(Some(list))
    }

    /// Save highscores to disk.
    ///
    /// Creates parent directories if they don't exist.
    pub async fn save(&self, list: &HighscoreList) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| YahtzeeError::SettingsIoError(e.to_string()))?;
        }
        let json = serde_json::to_string_pretty(list).map_err(|e| YahtzeeError::SettingsParseError(e.to_string()))?;
        let mut file = fs::File::create(&self.file_path)
            .await
            .map_err(|e| YahtzeeError::SettingsIoError(e.to_string()))?;
        file.write_all(json.as_bytes())
            .await
            .map_err(|e| YahtzeeError::SettingsIoError(e.to_string()))?;
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
    use crate::models::highscore_entry::HighscoreEntry;
    use crate::models::score::Score;

    #[tokio::test]
    async fn save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("yahtzee_test_highscore.json");
        let _ = std::fs::remove_file(&path);

        let store = HighscoreStore::new(&path);
        let mut list = HighscoreList::new();
        list.add(HighscoreEntry::new("Alice", Score::new(300)));
        list.add(HighscoreEntry::new("Bob", Score::new(200)));

        store.save(&list).await.unwrap();
        let loaded = store.load().await.unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.entries()[0].player_name(), "Alice");

        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn load_nonexistent_returns_none() {
        let path = std::env::temp_dir().join("yahtzee_nonexistent_highscore.json");
        let _ = std::fs::remove_file(&path);
        let store = HighscoreStore::new(&path);
        let loaded = store.load().await.unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn path_returns_file_path() {
        let store = HighscoreStore::new("/tmp/test_highscore.json");
        assert_eq!(store.path(), std::path::Path::new("/tmp/test_highscore.json"));
    }
}
