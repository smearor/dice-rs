use dice_rs::model::led::LedColorError;
use dice_rs::model::dice::DiceTypeError;
use dice_rs::error::DiceError;

/// Errors specific to the CLI tool.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// Invalid color string (from `LedColor::from_str`).
    #[error(transparent)]
    InvalidColor(#[from] LedColorError),

    /// Invalid dice type string (from `DiceType::from_str`).
    #[error(transparent)]
    InvalidDiceType(#[from] DiceTypeError),

    /// Invalid argument value.
    #[error("{0}")]
    InvalidArgument(String),

    /// No dice connected for a command that requires one.
    #[allow(dead_code)]
    #[error("no dice connected — use 'connect' first")]
    NotConnected,

    /// Underlying library error.
    #[error(transparent)]
    Dice(#[from] DiceError),

    /// I/O error (stdin read, etc.).
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// JSON serialize error (stdin read, etc.).
    #[error(transparent)]
    JsonSerialize(#[from] serde_json::Error),
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        Self::InvalidArgument(s)
    }
}

/// Convenience type alias.
pub type Result<T> = std::result::Result<T, CliError>;
