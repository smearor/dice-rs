use std::fs;
use std::io;
use std::path::PathBuf;

use tracing::debug;

use crate::config::app_settings::AppSettingsData;
use crate::config::dice_settings::DiceSettings;

/// Returns the config directory path: `~/.config/dice-rs/`.
fn config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("dice-rs");
    path
}

/// Returns the path to the app config file: `~/.config/dice-rs/config.toml`.
fn app_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

/// Returns the path to a per-dice config file: `~/.config/dice-rs/<device_name>.toml`.
fn dice_config_path(device_name: &str) -> PathBuf {
    config_dir().join(format!("{device_name}.toml"))
}

/// Ensures the config directory exists, creating it if necessary.
fn ensure_config_dir() -> io::Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

/// Load app settings from `~/.config/dice-rs/config.toml`.
///
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_app_settings() -> Option<AppSettingsData> {
    let path = app_config_path();
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            debug!(error = %error, path = ?path, "app config not found or unreadable");
            return None;
        }
    };
    match toml::from_str::<AppSettingsData>(&content) {
        Ok(data) => Some(data),
        Err(error) => {
            debug!(error = %error, path = ?path, "failed to parse app config");
            None
        }
    }
}

/// Save app settings to `~/.config/dice-rs/config.toml`.
pub fn save_app_settings(data: &AppSettingsData) {
    if let Err(error) = save_app_settings_inner(data) {
        debug!(error = %error, "failed to save app settings");
    }
}

fn save_app_settings_inner(data: &AppSettingsData) -> io::Result<()> {
    ensure_config_dir()?;
    let toml = toml::to_string(data).map_err(io::Error::other)?;
    fs::write(app_config_path(), toml)?;
    Ok(())
}

/// Load per-dice settings from `~/.config/dice-rs/<device_name>.toml`.
///
/// Returns `None` if the file does not exist or cannot be parsed.
pub fn load_dice_settings(device_name: &str) -> Option<DiceSettings> {
    let path = dice_config_path(device_name);
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            debug!(error = %error, path = ?path, "dice config not found or unreadable");
            return None;
        }
    };
    match toml::from_str::<DiceSettings>(&content) {
        Ok(data) => Some(data),
        Err(error) => {
            debug!(error = %error, path = ?path, "failed to parse dice config");
            None
        }
    }
}

/// Save per-dice settings to `~/.config/dice-rs/<device_name>.toml`.
pub fn save_dice_settings(device_name: &str, data: &DiceSettings) {
    if let Err(error) = save_dice_settings_inner(device_name, data) {
        debug!(error = %error, "failed to save dice settings");
    }
}

fn save_dice_settings_inner(device_name: &str, data: &DiceSettings) -> io::Result<()> {
    ensure_config_dir()?;
    let toml = toml::to_string(data).map_err(io::Error::other)?;
    fs::write(dice_config_path(device_name), toml)?;
    Ok(())
}
