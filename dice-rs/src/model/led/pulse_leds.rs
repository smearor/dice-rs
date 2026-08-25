use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// LED selection for the Pulse LEDs BLE command.
///
/// Controls which of the two LEDs participate in the pulse animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PulseLeds {
    /// Both LEDs pulse.
    #[default]
    Both,
    /// Only LED 1 pulses.
    Led1,
    /// Only LED 2 pulses.
    Led2,
}

impl PulseLeds {
    /// Converts the LED selection to its BLE byte value.
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Both => 0,
            Self::Led1 => 1,
            Self::Led2 => 2,
        }
    }
}

impl From<PulseLeds> for u8 {
    fn from(leds: PulseLeds) -> u8 {
        leds.as_u8()
    }
}

impl From<u8> for PulseLeds {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Both,
            1 => Self::Led1,
            _ => Self::Led2,
        }
    }
}

impl fmt::Display for PulseLeds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Both => write!(f, "both"),
            Self::Led1 => write!(f, "led1"),
            Self::Led2 => write!(f, "led2"),
        }
    }
}

impl FromStr for PulseLeds {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "both" | "0" => Ok(Self::Both),
            "led1" | "1" => Ok(Self::Led1),
            "led2" | "2" => Ok(Self::Led2),
            _ => Err(format!("unknown LED selection: '{s}' (expected 'both', 'led1', or 'led2')")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_is_zero() {
        assert_eq!(PulseLeds::Both.as_u8(), 0);
    }

    #[test]
    fn led1_is_one() {
        assert_eq!(PulseLeds::Led1.as_u8(), 1);
    }

    #[test]
    fn led2_is_two() {
        assert_eq!(PulseLeds::Led2.as_u8(), 2);
    }

    #[test]
    fn from_u8_both() {
        assert_eq!(PulseLeds::from(0u8), PulseLeds::Both);
    }

    #[test]
    fn from_u8_led1() {
        assert_eq!(PulseLeds::from(1u8), PulseLeds::Led1);
    }

    #[test]
    fn from_u8_led2() {
        assert_eq!(PulseLeds::from(2u8), PulseLeds::Led2);
    }

    #[test]
    fn from_u8_unknown_defaults_to_led2() {
        assert_eq!(PulseLeds::from(255u8), PulseLeds::Led2);
    }

    #[test]
    fn round_trip_both() {
        let leds = PulseLeds::Both;
        assert_eq!(PulseLeds::from(leds.as_u8()), leds);
    }

    #[test]
    fn round_trip_led1() {
        let leds = PulseLeds::Led1;
        assert_eq!(PulseLeds::from(leds.as_u8()), leds);
    }

    #[test]
    fn round_trip_led2() {
        let leds = PulseLeds::Led2;
        assert_eq!(PulseLeds::from(leds.as_u8()), leds);
    }
}
