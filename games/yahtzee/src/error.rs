use crate::models::category::ScoreCategory;

/// Errors that can occur during Yahtzee game operations.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum YahtzeeError {
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
}

/// Convenience type alias used throughout the crate.
pub type Result<T> = std::result::Result<T, YahtzeeError>;
