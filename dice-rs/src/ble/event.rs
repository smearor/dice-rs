use crate::ble::parse_error::ParseError;
use crate::error::Result;
use crate::model::acceleration::Acceleration;
use crate::model::dice::DiceColor;

/// Raw notification events from the GoDice.
///
/// Each variant corresponds to a specific BLE notification packet format.
/// See the BLE protocol reference for byte-level details.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Dice has started rolling (`0x52`).
    RollStart,
    /// Dice is stable and flat after a roll (`0x53` + XYZ).
    Stable { acceleration: Acceleration },
    /// Dice is stable after a fake roll (`0x46 0x53` + XYZ).
    FakeStable { acceleration: Acceleration },
    /// Dice is stable but tilted after a roll (`0x54 0x53` + XYZ).
    TiltStable { acceleration: Acceleration },
    /// Dice is stable after small movement (`0x4D 0x53` + XYZ).
    MoveStable { acceleration: Acceleration },
    /// Battery level response (`Bat` + level byte).
    BatteryLevel { level: u8 },
    /// Dice color response (`Col` + color byte).
    DiceColor { color: DiceColor },
    /// Calibration response (tentative: `Cal` + status byte).
    Calibrated { success: bool },
    /// Charging status notification (`Char` + charging byte: 0 = not charging, 1 = charging).
    Charging { charging: bool },
    /// Single tap detected (`Tap`).
    Tap,
    /// Double tap detected (`DTap`).
    DoubleTap,
}

impl Event {
    /// Parse a notification packet into an `Event`.
    ///
    /// Inspects the first byte(s) to determine the event type, then
    /// extracts the payload.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.is_empty() {
            return Err(ParseError::EmptyPacket.into());
        }

        let first = data[0];

        // RollStart: single byte 0x52 ('R')
        if first == 0x52 {
            return Ok(Self::RollStart);
        }

        // BatteryLevel: prefix "Bat" (0x42, 0x61, 0x74) + level byte
        if data.len() >= 4 && &data[0..3] == b"Bat" {
            return Ok(Self::BatteryLevel { level: data[3] });
        }

        // DiceColor: prefix "Col" (0x43, 0x6F, 0x6C) + color byte
        if data.len() >= 4 && &data[0..3] == b"Col" {
            let color = DiceColor::try_from(data[3]).map_err(|_| ParseError::InvalidColor(data[3]))?;
            return Ok(Self::DiceColor { color });
        }

        // Calibrated: prefix "Cal" (0x43, 0x61, 0x6C) + status byte
        // NOTE: Distinguished from DiceColor by second byte: 0x61 ('a') vs 0x6F ('o')
        if data.len() >= 4 && data[0] == 0x43 && data[1] == 0x61 && data[2] == 0x6C {
            return Ok(Self::Calibrated { success: data[3] != 0 });
        }

        // Charging: prefix "Char" (0x43, 0x68, 0x61, 0x72) + charging byte
        if data.len() >= 5 && &data[0..4] == b"Char" {
            return Ok(Self::Charging { charging: data[4] != 0 });
        }

        // Tap: prefix "Tap" (0x54, 0x61, 0x70)
        if data.len() >= 3 && &data[0..3] == b"Tap" {
            return Ok(Self::Tap);
        }

        // DoubleTap: prefix "DTap" (0x44, 0x54, 0x61, 0x70)
        if data.len() >= 4 && &data[0..4] == b"DTap" {
            return Ok(Self::DoubleTap);
        }

        // Stable: single byte 0x53 ('S') + 3 signed bytes XYZ
        if first == 0x53 {
            if data.len() < 4 {
                return Err(ParseError::TruncatedPacket {
                    expected: 4,
                    actual: data.len(),
                }
                .into());
            }
            return Ok(Self::Stable {
                acceleration: Acceleration::try_from(&data[1..4])?,
            });
        }

        // Two-byte prefix events: FS, TS, MS — all followed by 3 signed bytes XYZ
        if data.len() >= 5 && data[1] == 0x53 {
            let acceleration = Acceleration::try_from(&data[2..5])?;
            return match first {
                0x46 => Ok(Self::FakeStable { acceleration }),
                0x54 => Ok(Self::TiltStable { acceleration }),
                0x4D => Ok(Self::MoveStable { acceleration }),
                _ => Err(ParseError::UnknownEvent { byte: first }.into()),
            };
        }

        Err(ParseError::UnknownEvent { byte: first }.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_start() {
        assert_eq!(Event::parse(&[0x52]), Ok(Event::RollStart));
    }

    #[test]
    fn stable_event() {
        let data = [0x53, 10, 20, 30];
        let event = Event::parse(&data).unwrap();
        assert_eq!(
            event,
            Event::Stable {
                acceleration: Acceleration::try_from(&[10u8, 20, 30][..]).unwrap()
            }
        );
    }

    #[test]
    fn fake_stable_event() {
        let data = [0x46, 0x53, 1, 2, 3];
        let event = Event::parse(&data).unwrap();
        assert_eq!(
            event,
            Event::FakeStable {
                acceleration: Acceleration::try_from(&[1u8, 2, 3][..]).unwrap()
            }
        );
    }

    #[test]
    fn tilt_stable_event() {
        let data = [0x54, 0x53, 1, 2, 3];
        let event = Event::parse(&data).unwrap();
        assert_eq!(
            event,
            Event::TiltStable {
                acceleration: Acceleration::try_from(&[1u8, 2, 3][..]).unwrap()
            }
        );
    }

    #[test]
    fn move_stable_event() {
        let data = [0x4D, 0x53, 1, 2, 3];
        let event = Event::parse(&data).unwrap();
        assert_eq!(
            event,
            Event::MoveStable {
                acceleration: Acceleration::try_from(&[1u8, 2, 3][..]).unwrap()
            }
        );
    }

    #[test]
    fn battery_level_event() {
        let data = [0x42, 0x61, 0x74, 75];
        assert_eq!(Event::parse(&data), Ok(Event::BatteryLevel { level: 75 }));
    }

    #[test]
    fn dice_color_event() {
        let data = [0x43, 0x6F, 0x6C, 2];
        assert_eq!(Event::parse(&data), Ok(Event::DiceColor { color: DiceColor::Green }));
    }

    #[test]
    fn calibrated_event_success() {
        let data = [0x43, 0x61, 0x6C, 0x01];
        assert_eq!(Event::parse(&data), Ok(Event::Calibrated { success: true }));
    }

    #[test]
    fn calibrated_event_failure() {
        let data = [0x43, 0x61, 0x6C, 0x00];
        assert_eq!(Event::parse(&data), Ok(Event::Calibrated { success: false }));
    }

    #[test]
    fn calibrated_distinguishes_from_dice_color() {
        let color_data = [0x43, 0x6F, 0x6C, 2];
        let cal_data = [0x43, 0x61, 0x6C, 1];
        assert!(matches!(Event::parse(&color_data), Ok(Event::DiceColor { .. })));
        assert!(matches!(Event::parse(&cal_data), Ok(Event::Calibrated { .. })));
    }

    #[test]
    fn empty_packet() {
        assert!(Event::parse(&[]).is_err());
    }

    #[test]
    fn unknown_event() {
        assert!(Event::parse(&[0xFF]).is_err());
    }

    #[test]
    fn charging_event_on() {
        let data = [0x43, 0x68, 0x61, 0x72, 0x01];
        assert_eq!(Event::parse(&data), Ok(Event::Charging { charging: true }));
    }

    #[test]
    fn charging_event_off() {
        let data = [0x43, 0x68, 0x61, 0x72, 0x00];
        assert_eq!(Event::parse(&data), Ok(Event::Charging { charging: false }));
    }

    #[test]
    fn tap_event() {
        let data = [0x54, 0x61, 0x70];
        assert_eq!(Event::parse(&data), Ok(Event::Tap));
    }

    #[test]
    fn double_tap_event() {
        let data = [0x44, 0x54, 0x61, 0x70];
        assert_eq!(Event::parse(&data), Ok(Event::DoubleTap));
    }

    #[test]
    fn truncated_stable() {
        assert!(Event::parse(&[0x53, 1]).is_err());
    }
}
