use crate::models::category::ScoreCategory;

/// Errors that can occur during Yatzy game operations.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum YatzyError {
    /// A scorecard category was already filled or crossed out.
    #[error("category {0:?} is already filled")]
    CategoryAlreadyFilled(ScoreCategory),
    /// A dice slot index was out of range (valid: 0-4).
    #[error("invalid dice slot: {0} (valid: 0-4)")]
    InvalidDiceSlot(u8),
    /// A roll count was out of range (valid: 1-3).
    #[error("invalid roll count: {0} (valid: 1-3)")]
    InvalidRollCount(u8),
    /// A round number was out of range (valid: 1-13).
    #[error("invalid round number: {0} (valid: 1-13)")]
    InvalidRoundNumber(u8),
    /// A player index was out of range.
    #[error("invalid player index: {0}")]
    InvalidPlayerIndex(usize),
    /// A player name was empty.
    #[error("player name cannot be empty")]
    EmptyPlayerName,
    /// The game is already over.
    #[error("game is over")]
    GameOver,
    /// Not enough players to start a game (minimum: 1).
    #[error("not enough players: {0} (minimum: 1)")]
    NotEnoughPlayers(usize),
    /// A face value was invalid (0).
    #[error("invalid face value: {0}")]
    InvalidFaceValue(u8),
    /// A BLE operation failed (scan, connect, reconnect, LED command).
    #[error("BLE error: {0}")]
    Ble(String),
    /// A mutex lock was poisoned by a panicking thread.
    #[error("lock poisoned")]
    LockPoisoned,
    /// A dice was not found in the slot mapping.
    #[error("dice not found in slot {0}")]
    DiceNotFound(u8),
    /// Not all 5 dice slots are assigned.
    #[error("not all dice assigned: {0}/5")]
    NotAllDiceAssigned(usize),
    /// An action was attempted when it's not the player's turn.
    #[error("not player {0}'s turn")]
    NotYourTurn(usize),
    /// An action was attempted in the wrong turn phase.
    #[error("action not allowed in phase {0:?} (expected {1:?})")]
    PhaseMismatch(String, String),
    /// An action was attempted but dice are not connected.
    #[error("dice not connected: {0}/5")]
    DiceNotConnected(usize),
    /// An action was attempted after the game is over.
    #[error("game is already over")]
    GameAlreadyOver,
    /// A probability value was outside the valid range [0.0, 1.0].
    #[error("invalid probability value: {0}")]
    InvalidProbability(f64),
    /// A player count was outside the valid range [1, 6].
    #[error("invalid player count: {0} (valid: 1-6)")]
    InvalidPlayerCount(u8),
    /// Too many players for a game (maximum: 6).
    #[error("too many players: {0} (maximum: 6)")]
    TooManyPlayers(usize),
    /// Two players were assigned the same color.
    #[error("duplicate player color: {0}")]
    DuplicateColor(String),
    /// Two players were assigned the same name.
    #[error("duplicate player name: {0}")]
    DuplicateName(String),
    /// A settings file I/O operation failed.
    #[error("settings I/O error: {0}")]
    SettingsIoError(String),
    /// A settings file could not be parsed.
    #[error("settings parse error: {0}")]
    SettingsParseError(String),
    /// A dice reconnection attempt failed.
    #[error("reconnection failed for device: {0}")]
    ReconnectionFailed(String),
}

/// Convenience type alias used throughout the crate.
pub type Result<T> = std::result::Result<T, YatzyError>;
