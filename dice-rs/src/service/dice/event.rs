use crate::model::acceleration::Acceleration;
use crate::model::charging_state::ChargingState;
use crate::model::face::FaceValue;
use crate::model::stability_descriptor::StabilityDescriptor;
use serde::Deserialize;
use serde::Serialize;

/// High-level events emitted by a connected GoDice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
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
    /// Dice charging status changed.
    Charging { state: ChargingState },
    /// Single tap detected.
    Tap,
    /// Double tap detected.
    DoubleTap,
    /// Dice has disconnected.
    Disconnected,
}

impl DiceEvent {
    /// Returns the stability descriptor for stable/rolling events.
    pub fn stability(&self) -> Option<StabilityDescriptor> {
        match self {
            Self::RollStart => Some(StabilityDescriptor::Rolling),
            Self::Stable { .. } => Some(StabilityDescriptor::Stable),
            Self::TiltStable { .. } => Some(StabilityDescriptor::TiltStable),
            Self::FakeStable { .. } => Some(StabilityDescriptor::FakeStable),
            Self::MoveStable { .. } => Some(StabilityDescriptor::MoveStable),
            Self::Charging { .. } => None,
            Self::Tap => None,
            Self::DoubleTap => None,
            Self::Disconnected => None,
        }
    }
}

impl std::fmt::Display for DiceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RollStart => write!(f, "rolling"),
            Self::Stable { face, .. } => write!(f, "stable face={face}"),
            Self::TiltStable { face, .. } => write!(f, "tilt-stable face={face}"),
            Self::FakeStable { face, .. } => write!(f, "fake-stable face={face}"),
            Self::MoveStable { face, .. } => write!(f, "move-stable face={face}"),
            Self::Charging { state } => write!(f, "charging={state}"),
            Self::Tap => write!(f, "tap"),
            Self::DoubleTap => write!(f, "double-tap"),
            Self::Disconnected => write!(f, "disconnected"),
        }
    }
}
