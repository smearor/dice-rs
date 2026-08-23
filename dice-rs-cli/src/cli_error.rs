/// Errors specific to the CLI tool.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// Device with the given address was not found in scan results.
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    /// Invalid color string.
    #[error("invalid color: {0} (expected named color or hex like FF0000)")]
    InvalidColor(String),

    /// Invalid dice type string.
    #[error("invalid dice type: {0} (expected d6, d20, d10, d10x, d4, d8, or d12)")]
    InvalidDiceType(String),

    /// No dice connected for a command that requires one.
    #[allow(dead_code)]
    #[error("no dice connected — use 'connect' first")]
    NotConnected,

    /// Underlying library error.
    #[error(transparent)]
    Dice(#[from] dice_rs::error::DiceError),

    /// I/O error (stdin read, etc.).
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Convenience type alias.
pub type Result<T> = std::result::Result<T, CliError>;
