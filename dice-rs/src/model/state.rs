/// Describes the stability state of the dice.
///
/// Maps the raw BLE events to a high-level stability classification
/// so applications can distinguish stability types without matching on
/// `DiceEvent` variants directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityDescriptor {
    /// Dice is currently rolling (RollStart event).
    Rolling,
    /// Dice is stable and flat (Stable event).
    Stable,
    /// Dice is stable but tilted (TiltStable event).
    TiltStable,
    /// Dice is stable after a fake roll (FakeStable event).
    FakeStable,
    /// Dice is stable after small movement (MoveStable event).
    MoveStable,
}

impl std::fmt::Display for StabilityDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rolling => write!(f, "Rolling"),
            Self::Stable => write!(f, "Stable"),
            Self::TiltStable => write!(f, "TiltStable"),
            Self::FakeStable => write!(f, "FakeStable"),
            Self::MoveStable => write!(f, "MoveStable"),
        }
    }
}

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
    fn stability_display() {
        assert_eq!(StabilityDescriptor::Rolling.to_string(), "Rolling");
        assert_eq!(StabilityDescriptor::TiltStable.to_string(), "TiltStable");
    }

    #[test]
    fn dice_state_display() {
        assert_eq!(DiceState::Connected.to_string(), "Connected");
        assert_eq!(DiceState::Disconnected.to_string(), "Disconnected");
    }
}
