use crate::ble::command_error::CommandError;
use crate::model::led::LedColor;
use crate::model::led::PulseBlinkMode;
use crate::model::led::PulseLeds;

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
        blink_mode: PulseBlinkMode,
        leds: PulseLeds,
    },
    /// Stop any active pulse LED animation.
    StopPulseLeds,
    /// Request dice color. Response: `Event::DiceColor`.
    GetDiceColor,
    /// Initialize dice with sensitivity and LED configuration.
    Init {
        sensitivity: u8,
        pulse_count: u8,
        on_time: u8,
        off_time: u8,
        color: LedColor,
        blink_mode: PulseBlinkMode,
        leds: PulseLeds,
    },
    /// Update roll detection sensitivity parameters.
    DetectionSettings {
        samples_count: u8,
        movement_count: u8,
        face_count: u8,
        min_flat_deg: u8,
        max_flat_deg: u8,
        weak_stable: u8,
        movement_deg: u8,
        roll_threshold: u8,
    },
    /// Enable or disable single tap interrupt notifications.
    /// `true` enables, `false` disables.
    SetTapInterrupt { enabled: bool },
    /// Enable or disable double tap interrupt notifications.
    /// `true` enables, `false` disables.
    SetDoubleTapInterrupt { enabled: bool },
    /// Hardware calibration. Opcode 0x13 — tentative, protocol not yet confirmed.
    /// Response: `Event::Calibrated`.
    Calibrate,
}

/// BLE opcodes and encoding for each command variant.
impl Command {
    const OPCODE_GET_BATTERY_LEVEL: u8 = 0x03;
    const OPCODE_SET_LEDS: u8 = 0x08;
    const OPCODE_PULSE_LEDS: u8 = 0x10;
    const OPCODE_STOP_PULSE_LEDS: u8 = 0x14;
    const OPCODE_GET_DICE_COLOR: u8 = 0x17;
    const OPCODE_INIT: u8 = 0x19;
    const OPCODE_SET_TAP_INTERRUPT: u8 = 0x31;
    const OPCODE_SET_DOUBLE_TAP_INTERRUPT: u8 = 0x32;
    const OPCODE_DETECTION_SETTINGS: u8 = 0x65;
    const OPCODE_CALIBRATE: u8 = 0x13;

    const LEN_GET_BATTERY_LEVEL: usize = 1;
    const LEN_SET_LEDS: usize = 7;
    const LEN_PULSE_LEDS: usize = 9;
    const LEN_STOP_PULSE_LEDS: usize = 1;
    const LEN_GET_DICE_COLOR: usize = 1;
    const LEN_INIT: usize = 10;
    const LEN_SET_TAP_INTERRUPT: usize = 2;
    const LEN_SET_DOUBLE_TAP_INTERRUPT: usize = 2;
    const LEN_DETECTION_SETTINGS: usize = 9;
    const LEN_CALIBRATE: usize = 1;

    /// Returns the opcode byte for this command.
    pub const fn opcode(&self) -> u8 {
        match self {
            Self::GetBatteryLevel => Self::OPCODE_GET_BATTERY_LEVEL,
            Self::SetLeds { .. } => Self::OPCODE_SET_LEDS,
            Self::PulseLeds { .. } => Self::OPCODE_PULSE_LEDS,
            Self::StopPulseLeds => Self::OPCODE_STOP_PULSE_LEDS,
            Self::GetDiceColor => Self::OPCODE_GET_DICE_COLOR,
            Self::Init { .. } => Self::OPCODE_INIT,
            Self::SetTapInterrupt { .. } => Self::OPCODE_SET_TAP_INTERRUPT,
            Self::SetDoubleTapInterrupt { .. } => Self::OPCODE_SET_DOUBLE_TAP_INTERRUPT,
            Self::DetectionSettings { .. } => Self::OPCODE_DETECTION_SETTINGS,
            Self::Calibrate => Self::OPCODE_CALIBRATE,
        }
    }

    /// Returns the expected total packet length (opcode + payload bytes).
    pub const fn data_len(&self) -> usize {
        match self {
            Self::GetBatteryLevel => Self::LEN_GET_BATTERY_LEVEL,
            Self::SetLeds { .. } => Self::LEN_SET_LEDS,
            Self::PulseLeds { .. } => Self::LEN_PULSE_LEDS,
            Self::StopPulseLeds => Self::LEN_STOP_PULSE_LEDS,
            Self::GetDiceColor => Self::LEN_GET_DICE_COLOR,
            Self::Init { .. } => Self::LEN_INIT,
            Self::SetTapInterrupt { .. } => Self::LEN_SET_TAP_INTERRUPT,
            Self::SetDoubleTapInterrupt { .. } => Self::LEN_SET_DOUBLE_TAP_INTERRUPT,
            Self::DetectionSettings { .. } => Self::LEN_DETECTION_SETTINGS,
            Self::Calibrate => Self::LEN_CALIBRATE,
        }
    }
}

/// Encode a `Command` into its BLE byte representation.
impl From<Command> for Vec<u8> {
    fn from(command: Command) -> Self {
        let mut data = vec![command.opcode()];
        match command {
            Command::GetBatteryLevel => {}
            Command::SetLeds { led1, led2 } => {
                data.extend_from_slice(&[led1.r, led1.g, led1.b, led2.r, led2.g, led2.b]);
            }
            Command::PulseLeds {
                pulse_count,
                on_time,
                off_time,
                color,
                blink_mode,
                leds,
            } => {
                data.extend_from_slice(&[pulse_count, on_time, off_time, color.r, color.g, color.b, blink_mode.as_u8(), leds.as_u8()]);
            }
            Command::StopPulseLeds => {}
            Command::GetDiceColor => {}
            Command::Init {
                sensitivity,
                pulse_count,
                on_time,
                off_time,
                color,
                blink_mode,
                leds,
            } => {
                data.extend_from_slice(&[sensitivity, pulse_count, on_time, off_time, color.r, color.g, color.b, blink_mode.as_u8(), leds.as_u8()]);
            }
            Command::DetectionSettings {
                samples_count,
                movement_count,
                face_count,
                min_flat_deg,
                max_flat_deg,
                weak_stable,
                movement_deg,
                roll_threshold,
            } => {
                data.extend_from_slice(&[samples_count, movement_count, face_count, min_flat_deg, max_flat_deg, weak_stable, movement_deg, roll_threshold]);
            }
            Command::SetTapInterrupt { enabled } => {
                data.push(u8::from(enabled));
            }
            Command::SetDoubleTapInterrupt { enabled } => {
                data.push(u8::from(enabled));
            }
            Command::Calibrate => {}
        }
        data
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
            Self::OPCODE_GET_BATTERY_LEVEL if data.len() == Self::LEN_GET_BATTERY_LEVEL => Ok(Self::GetBatteryLevel),
            Self::OPCODE_SET_LEDS if data.len() == Self::LEN_SET_LEDS => Ok(Self::SetLeds {
                led1: LedColor::new(data[1], data[2], data[3]),
                led2: LedColor::new(data[4], data[5], data[6]),
            }),
            Self::OPCODE_PULSE_LEDS if data.len() == Self::LEN_PULSE_LEDS => Ok(Self::PulseLeds {
                pulse_count: data[1],
                on_time: data[2],
                off_time: data[3],
                color: LedColor::new(data[4], data[5], data[6]),
                blink_mode: PulseBlinkMode::from(data[7]),
                leds: PulseLeds::from(data[8]),
            }),
            Self::OPCODE_STOP_PULSE_LEDS if data.len() == Self::LEN_STOP_PULSE_LEDS => Ok(Self::StopPulseLeds),
            Self::OPCODE_GET_DICE_COLOR if data.len() == Self::LEN_GET_DICE_COLOR => Ok(Self::GetDiceColor),
            Self::OPCODE_INIT if data.len() == Self::LEN_INIT => Ok(Self::Init {
                sensitivity: data[1],
                pulse_count: data[2],
                on_time: data[3],
                off_time: data[4],
                color: LedColor::new(data[5], data[6], data[7]),
                blink_mode: PulseBlinkMode::from(data[8]),
                leds: PulseLeds::from(data[9]),
            }),
            Self::OPCODE_DETECTION_SETTINGS if data.len() == Self::LEN_DETECTION_SETTINGS => Ok(Self::DetectionSettings {
                samples_count: data[1],
                movement_count: data[2],
                face_count: data[3],
                min_flat_deg: data[4],
                max_flat_deg: data[5],
                weak_stable: data[6],
                movement_deg: data[7],
                roll_threshold: data[8],
            }),
            Self::OPCODE_SET_TAP_INTERRUPT if data.len() == Self::LEN_SET_TAP_INTERRUPT => Ok(Self::SetTapInterrupt { enabled: data[1] != 0 }),
            Self::OPCODE_SET_DOUBLE_TAP_INTERRUPT if data.len() == Self::LEN_SET_DOUBLE_TAP_INTERRUPT => Ok(Self::SetDoubleTapInterrupt { enabled: data[1] != 0 }),
            Self::OPCODE_CALIBRATE if data.len() == Self::LEN_CALIBRATE => Ok(Self::Calibrate),
            opcode => Err(CommandError::UnknownOpcode { opcode, length: data.len() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_values() {
        assert_eq!(Command::GetBatteryLevel.opcode(), 0x03);
        assert_eq!(Command::SetLeds { led1: LedColor::RED, led2: LedColor::RED }.opcode(), 0x08);
        assert_eq!(Command::PulseLeds { pulse_count: 1, on_time: 1, off_time: 1, color: LedColor::RED, blink_mode: PulseBlinkMode::Color, leds: PulseLeds::Both }.opcode(), 0x10);
        assert_eq!(Command::StopPulseLeds.opcode(), 0x14);
        assert_eq!(Command::GetDiceColor.opcode(), 0x17);
        assert_eq!(Command::Init { sensitivity: 1, pulse_count: 1, on_time: 1, off_time: 1, color: LedColor::RED, blink_mode: PulseBlinkMode::Color, leds: PulseLeds::Both }.opcode(), 0x19);
        assert_eq!(Command::SetTapInterrupt { enabled: true }.opcode(), 0x31);
        assert_eq!(Command::SetDoubleTapInterrupt { enabled: true }.opcode(), 0x32);
        assert_eq!(Command::DetectionSettings { samples_count: 1, movement_count: 1, face_count: 6, min_flat_deg: 1, max_flat_deg: 1, weak_stable: 1, movement_deg: 1, roll_threshold: 1 }.opcode(), 0x65);
        assert_eq!(Command::Calibrate.opcode(), 0x13);
    }

    #[test]
    fn data_len_values() {
        assert_eq!(Command::GetBatteryLevel.data_len(), 1);
        assert_eq!(Command::SetLeds { led1: LedColor::RED, led2: LedColor::RED }.data_len(), 7);
        assert_eq!(Command::PulseLeds { pulse_count: 1, on_time: 1, off_time: 1, color: LedColor::RED, blink_mode: PulseBlinkMode::Color, leds: PulseLeds::Both }.data_len(), 9);
        assert_eq!(Command::StopPulseLeds.data_len(), 1);
        assert_eq!(Command::GetDiceColor.data_len(), 1);
        assert_eq!(Command::Init { sensitivity: 1, pulse_count: 1, on_time: 1, off_time: 1, color: LedColor::RED, blink_mode: PulseBlinkMode::Color, leds: PulseLeds::Both }.data_len(), 10);
        assert_eq!(Command::SetTapInterrupt { enabled: true }.data_len(), 2);
        assert_eq!(Command::SetDoubleTapInterrupt { enabled: true }.data_len(), 2);
        assert_eq!(Command::DetectionSettings { samples_count: 1, movement_count: 1, face_count: 6, min_flat_deg: 1, max_flat_deg: 1, weak_stable: 1, movement_deg: 1, roll_threshold: 1 }.data_len(), 9);
        assert_eq!(Command::Calibrate.data_len(), 1);
    }

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
            blink_mode: PulseBlinkMode::Color,
            leds: PulseLeds::Both,
        };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x10, 5, 10, 10, 0, 255, 0, 1, 0]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn stop_pulse_leds_round_trip() {
        let command = Command::StopPulseLeds;
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x14]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn init_round_trip() {
        let color = LedColor::new(255, 0, 0);
        let command = Command::Init {
            sensitivity: 30,
            pulse_count: 5,
            on_time: 10,
            off_time: 10,
            color,
            blink_mode: PulseBlinkMode::Color,
            leds: PulseLeds::Both,
        };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x19, 30, 5, 10, 10, 255, 0, 0, 1, 0]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn detection_settings_round_trip() {
        let command = Command::DetectionSettings {
            samples_count: 10,
            movement_count: 5,
            face_count: 6,
            min_flat_deg: 30,
            max_flat_deg: 60,
            weak_stable: 15,
            movement_deg: 20,
            roll_threshold: 25,
        };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x65, 10, 5, 6, 30, 60, 15, 20, 25]);
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
        assert_eq!(data, vec![0x13]);
    }

    #[test]
    fn calibrate_round_trip() {
        let command = Command::Calibrate;
        let encoded: Vec<u8> = command.clone().into();
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn set_tap_interrupt_round_trip() {
        let command = Command::SetTapInterrupt { enabled: true };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x31, 0x01]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);

        let command = Command::SetTapInterrupt { enabled: false };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x31, 0x00]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn set_double_tap_interrupt_round_trip() {
        let command = Command::SetDoubleTapInterrupt { enabled: true };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x32, 0x01]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);

        let command = Command::SetDoubleTapInterrupt { enabled: false };
        let encoded: Vec<u8> = command.clone().into();
        assert_eq!(encoded, vec![0x32, 0x00]);
        let decoded = Command::try_from(&encoded[..]).unwrap();
        assert_eq!(decoded, command);
    }
}
