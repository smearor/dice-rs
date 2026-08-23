/// Errors that can occur when parsing a notification packet.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    /// The packet is empty.
    #[error("empty packet")]
    EmptyPacket,
    /// The packet is shorter than expected.
    #[error("truncated packet: expected {expected} bytes, got {actual}")]
    TruncatedPacket { expected: usize, actual: usize },
    /// The first byte does not match any known event.
    #[error("unknown event byte: 0x{byte:02X}")]
    UnknownEvent { byte: u8 },
    /// The color byte is not a valid DiceColor value.
    #[error("invalid dice color value: {0}")]
    InvalidColor(u8),
}
