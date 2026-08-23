/// Error returned when an invalid dice type byte or string is encountered.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiceTypeError {
    /// The byte does not correspond to any known `DiceType`.
    #[error("invalid dice type byte: {0}")]
    InvalidValue(u8),
    /// The string does not correspond to any known `DiceType`.
    #[error("invalid dice type: {0} (expected d6, d20, d10, d10x, d4, d8, or d12)")]
    InvalidName(String),
}
