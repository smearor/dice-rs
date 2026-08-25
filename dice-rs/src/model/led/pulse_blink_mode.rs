use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// Blink mode for the Pulse LEDs BLE command.
///
/// Controls whether the pulse animation uses the specified color
/// or cycles through multiple colors (rainbow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PulseBlinkMode {
    /// Rainbow — pulse cycles through multiple colors.
    Rainbow,
    /// Color — pulse uses the specified LED color.
    #[default]
    Color,
}

impl PulseBlinkMode {
    /// Converts the blink mode to its BLE byte value.
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Rainbow => 0,
            Self::Color => 1,
        }
    }
}

impl From<PulseBlinkMode> for u8 {
    fn from(mode: PulseBlinkMode) -> u8 {
        mode.as_u8()
    }
}

impl From<u8> for PulseBlinkMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Rainbow,
            _ => Self::Color,
        }
    }
}

impl fmt::Display for PulseBlinkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rainbow => write!(f, "rainbow"),
            Self::Color => write!(f, "color"),
        }
    }
}

impl FromStr for PulseBlinkMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rainbow" | "0" => Ok(Self::Rainbow),
            "color" | "1" => Ok(Self::Color),
            _ => Err(format!("unknown blink mode: '{s}' (expected 'rainbow' or 'color')")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rainbow_is_zero() {
        assert_eq!(PulseBlinkMode::Rainbow.as_u8(), 0);
    }

    #[test]
    fn color_is_one() {
        assert_eq!(PulseBlinkMode::Color.as_u8(), 1);
    }

    #[test]
    fn from_u8_rainbow() {
        assert_eq!(PulseBlinkMode::from(0u8), PulseBlinkMode::Rainbow);
    }

    #[test]
    fn from_u8_color() {
        assert_eq!(PulseBlinkMode::from(1u8), PulseBlinkMode::Color);
    }

    #[test]
    fn from_u8_unknown_defaults_to_color() {
        assert_eq!(PulseBlinkMode::from(255u8), PulseBlinkMode::Color);
    }

    #[test]
    fn round_trip_rainbow() {
        let mode = PulseBlinkMode::Rainbow;
        assert_eq!(PulseBlinkMode::from(mode.as_u8()), mode);
    }

    #[test]
    fn round_trip_color() {
        let mode = PulseBlinkMode::Color;
        assert_eq!(PulseBlinkMode::from(mode.as_u8()), mode);
    }
}
