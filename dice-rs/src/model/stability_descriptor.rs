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

impl StabilityDescriptor {
    /// Return a short lowercase label describing the stability state.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Rolling => "rolling",
            Self::Stable => "stable",
            Self::TiltStable => "tilt",
            Self::FakeStable => "fake",
            Self::MoveStable => "move",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(StabilityDescriptor::Rolling.to_string(), "Rolling");
        assert_eq!(StabilityDescriptor::TiltStable.to_string(), "TiltStable");
    }
}
