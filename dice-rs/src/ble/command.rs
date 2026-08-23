use crate::ble::command_error::CommandError;
use crate::model::led::LedColor;

/// Tentative calibration opcode — protocol not yet confirmed.
const CALIBRATION_OPCODE: u8 = 0x13;

/// Commands sent to the GoDice via the NUS write characteristic.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Request battery level. Response: `Event::BatteryLevel`.
    GetBatteryLevel,
    /// Set both RGB LEDs. `(0, 0, 0)` turns an LED off.
    SetLeds { led1: LedColor, led2: LedColor },
    /// Pulse both LEDs with a color for a defined number of cycles.
    PulseLeds {
        pulse_count: u8,
        on_time: u8,
        off_time: u8,
        color: LedColor,
    },
    /// Request dice color. Response: `Event::DiceColor`.
    GetDiceColor,
    /// Hardware calibration (tentative — opcode 0x13 unconfirmed).
    /// Response: `Event::Calibrated`.
    Calibrate,
}

/// Encode a `Command` into its BLE byte representation.
impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        match command {
            Command::GetBatteryLevel => vec![0x03],
            Command::SetLeds { led1, led2 } => {
                vec![0x08, led1.r, led1.g, led1.b, led2.r, led2.g, led2.b]
            }
            Command::PulseLeds {
                pulse_count,
                on_time,
                off_time,
                color,
            } => {
                vec![0x10, pulse_count, on_time, off_time, color.r, color.g, color.b, 1, 0]
            }
            Command::GetDiceColor => vec![0x17],
            Command::Calibrate => vec![CALIBRATION_OPCODE],
        }
    }
}

/// Decode a `Command` from its BLE byte representation.
///
/// Useful for testing and protocol debugging.
impl TryFrom<&[u8]> for Command {
    type Error = CommandError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(CommandError::EmptyPacket);
        }
        match data[0] {
            0x03 if data.len() == 1 => Ok(Self::GetBatteryLevel),
            0x08 if data.len() == 7 => Ok(Self::SetLeds {
                led1: LedColor::new(data[1], data[2], data[3]),
                led2: LedColor::new(data[4], data[5], data[6]),
            }),
            0x10 if data.len() == 9 => Ok(Self::PulseLeds {
                pulse_count: data[1],
                on_time: data[2],
                off_time: data[3],
                color: LedColor::new(data[4], data[5], data[6]),
            }),
            0x17 if data.len() == 1 => Ok(Self::GetDiceColor),
            CALIBRATION_OPCODE if data.len() == 1 => Ok(Self::Calibrate),
            opcode => Err(CommandError::UnknownOpcode { opcode, length: data.len() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_battery_level_encode() {
        let data: Vec<u8> = Command::GetBatteryLevel.into();
        assert_eq!(data, vec![0x03]);
    }

    #[test]
    fn get_dice_color_encode() {
        let data: Vec<u8> = Command::GetDiceColor.into();
        assert_eq!(data, vec![0x17]);
    }

    #[test]
    fn set_leds_round_trip() {
        let led1 = LedColor::new(255, 128, 0);
        let led2 = LedColor::new(0, 64, 200);
        let command = Command::SetLeds { led1, led2 };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x08, 255, 128, 0, 0, 64, 200]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn pulse_leds_round_trip() {
        let color = LedColor::new(0, 255, 0);
        let command = Command::PulseLeds {
            pulse_count: 5,
            on_time: 10,
            off_time: 10,
            color,
        };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x10, 5, 10, 10, 0, 255, 0, 1, 0]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn decode_empty_packet() {
        assert!(Command::try_from(&[][..]).is_err());
    }

    #[test]
    fn decode_unknown_opcode() {
        assert!(Command::try_from(&[0xFF][..]).is_err());
    }

    #[test]
    fn decode_invalid_length_set_leds() {
        assert!(Command::try_from(&[0x08, 1, 2, 3][..]).is_err());
    }

    #[test]
    fn decode_invalid_length_pulse_leds() {
        assert!(Command::try_from(&[0x10, 5, 10, 10, 0, 255, 0][..]).is_err());
    }

    #[test]
    fn led_color_constants() {
        assert!(LedColor::OFF.is_off());
        assert!(!LedColor::RED.is_off());
        assert_eq!(LedColor::from_hex(0xFF8800), LedColor::new(255, 136, 0));
        assert_eq!(LedColor::RED.to_string(), "#FF0000");
    }

    #[test]
    fn calibrate_encode() {
        let data: Vec<u8> = Command::Calibrate.into();
        assert_eq!(data, vec![CALIBRATION_OPCODE]);
    }

    #[test]
    fn calibrate_round_trip() {
        let command = Command::Calibrate;
        let encoded: Vec<u8> = command.clone().into();
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }
}
