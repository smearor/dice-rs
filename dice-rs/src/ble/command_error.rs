/// Errors that can occur when encoding or decoding commands.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CommandError {
    /// The packet is empty.
    #[error("empty packet")]
    EmptyPacket,
    /// The opcode is not a known command.
    #[error("unknown opcode: 0x{opcode:02X} (length {length})")]
    UnknownOpcode { opcode: u8, length: usize },
    /// The packet length does not match the expected payload size.
    #[error("invalid payload length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_packet_display() {
        assert_eq!(CommandError::EmptyPacket.to_string(), "empty packet");
    }

    #[test]
    fn unknown_opcode_display() {
        let error = CommandError::UnknownOpcode { opcode: 0xFF, length: 1 };
        assert_eq!(error.to_string(), "unknown opcode: 0xFF (length 1)");
    }

    #[test]
    fn invalid_length_display() {
        let error = CommandError::InvalidLength { expected: 7, actual: 3 };
        assert_eq!(error.to_string(), "invalid payload length: expected 7, got 3");
    }
}
