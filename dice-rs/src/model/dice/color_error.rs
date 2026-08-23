/// Error returned when an invalid dice color byte is encountered.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiceColorError {
    /// The byte does not correspond to any known `DiceColor`.
    #[error("invalid dice color value: {0}")]
    InvalidValue(u8),
}
