/// Error returned when parsing a `LedColor` from a string.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LedColorError {
    /// The string is not a recognized color name or valid hex value.
    #[error("invalid color: {0} (expected named color or hex like FF0000)")]
    InvalidValue(String),
}
