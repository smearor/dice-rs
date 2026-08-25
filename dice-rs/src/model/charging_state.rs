use serde::Deserialize;
use serde::Serialize;

/// Charging state of a GoDice device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChargingState {
    /// Device is not charging.
    #[default]
    NotCharging,
    /// Device is charging.
    Charging,
}

impl From<bool> for ChargingState {
    fn from(charging: bool) -> Self {
        if charging { Self::Charging } else { Self::NotCharging }
    }
}

impl From<ChargingState> for bool {
    fn from(state: ChargingState) -> Self {
        matches!(state, ChargingState::Charging)
    }
}

impl std::fmt::Display for ChargingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCharging => write!(f, "not charging"),
            Self::Charging => write!(f, "charging"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bool_true() {
        assert_eq!(ChargingState::from(true), ChargingState::Charging);
    }

    #[test]
    fn from_bool_false() {
        assert_eq!(ChargingState::from(false), ChargingState::NotCharging);
    }

    #[test]
    fn into_bool_charging() {
        assert!(bool::from(ChargingState::Charging));
    }

    #[test]
    fn into_bool_not_charging() {
        assert!(!bool::from(ChargingState::NotCharging));
    }

    #[test]
    fn display_charging() {
        assert_eq!(ChargingState::Charging.to_string(), "charging");
    }

    #[test]
    fn display_not_charging() {
        assert_eq!(ChargingState::NotCharging.to_string(), "not charging");
    }
}
