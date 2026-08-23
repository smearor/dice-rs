use crate::model::acceleration::Acceleration;
use crate::model::face::FaceValue;

/// High-level events emitted by a connected GoDice.
#[derive(Debug, Clone, PartialEq)]
pub enum DiceEvent {
    /// Dice has started rolling.
    RollStart,
    /// Dice is stable and flat after a roll.
    Stable { face: FaceValue, acceleration: Acceleration },
    /// Dice is stable but tilted after a roll.
    TiltStable { face: FaceValue, acceleration: Acceleration },
    /// Dice is stable after a fake roll.
    FakeStable { face: FaceValue, acceleration: Acceleration },
    /// Dice is stable after a small movement (face rotation).
    MoveStable { face: FaceValue, acceleration: Acceleration },
    /// Dice has disconnected.
    Disconnected,
}

impl DiceEvent {
    /// Returns the stability descriptor for stable/rolling events.
    pub fn stability(&self) -> Option<crate::model::state::StabilityDescriptor> {
        use crate::model::state::StabilityDescriptor;
        match self {
            Self::RollStart => Some(StabilityDescriptor::Rolling),
            Self::Stable { .. } => Some(StabilityDescriptor::Stable),
            Self::TiltStable { .. } => Some(StabilityDescriptor::TiltStable),
            Self::FakeStable { .. } => Some(StabilityDescriptor::FakeStable),
            Self::MoveStable { .. } => Some(StabilityDescriptor::MoveStable),
            Self::Disconnected => None,
        }
    }
}
