/// Connection state of a dice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiceState {
    /// Dice is discovered but not connected.
    Discovered,
    /// Connection in progress.
    Connecting,
    /// Dice is connected and operational.
    Connected,
    /// Reconnection in progress.
    Reconnecting,
    /// Dice is disconnecting.
    Disconnecting,
    /// Dice is disconnected.
    Disconnected,
}

impl std::fmt::Display for DiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Discovered => write!(f, "Discovered"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected => write!(f, "Connected"),
            Self::Reconnecting => write!(f, "Reconnecting"),
            Self::Disconnecting => write!(f, "Disconnecting"),
            Self::Disconnected => write!(f, "Disconnected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(DiceState::Connected.to_string(), "Connected");
        assert_eq!(DiceState::Disconnected.to_string(), "Disconnected");
    }
}
