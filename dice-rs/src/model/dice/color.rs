use crate::model::dice::color_error::DiceColorError;
use serde::Deserialize;
use serde::Serialize;

/// Physical color of a GoDice device.
///
/// Encoded as a single byte in the `Col` response notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum DiceColor {
    /// Black shell.
    Black = 0,
    /// Red shell.
    Red = 1,
    /// Green shell.
    Green = 2,
    /// Blue shell.
    Blue = 3,
    /// Yellow shell.
    Yellow = 4,
    /// Orange shell.
    Orange = 5,
}

impl TryFrom<u8> for DiceColor {
    type Error = DiceColorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::Red),
            2 => Ok(Self::Green),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Orange),
            _ => Err(DiceColorError::InvalidValue(value)),
        }
    }
}

impl From<DiceColor> for u8 {
    fn from(color: DiceColor) -> Self {
        color as u8
    }
}

/// Parse a dice color from a single character code.
///
/// Used to extract the color from a GoDice device name (e.g. `GoDice_0D89BF_K_v04`).
/// Accepts: `K`=Black, `R`=Red, `G`=Green, `B`=Blue, `Y`=Yellow, `O`=Orange.
/// Case-insensitive.
impl TryFrom<char> for DiceColor {
    type Error = DiceColorError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch.to_ascii_uppercase() {
            'K' => Ok(Self::Black),
            'R' => Ok(Self::Red),
            'G' => Ok(Self::Green),
            'B' => Ok(Self::Blue),
            'Y' => Ok(Self::Yellow),
            'O' => Ok(Self::Orange),
            _ => Err(DiceColorError::InvalidCharacter(ch)),
        }
    }
}

impl std::fmt::Display for DiceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Blue => write!(f, "Blue"),
            Self::Yellow => write!(f, "Yellow"),
            Self::Orange => write!(f, "Orange"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid() {
        assert_eq!(DiceColor::try_from(0), Ok(DiceColor::Black));
        assert_eq!(DiceColor::try_from(5), Ok(DiceColor::Orange));
    }

    #[test]
    fn try_from_invalid() {
        assert!(DiceColor::try_from(6).is_err());
    }

    #[test]
    fn from_to_u8_roundtrip() {
        for value in 0..=5 {
            let color = DiceColor::try_from(value).expect("valid color");
            assert_eq!(u8::from(color), value);
        }
    }

    #[test]
    fn display() {
        assert_eq!(DiceColor::Red.to_string(), "Red");
    }

    #[test]
    fn try_from_char_valid() {
        assert_eq!(DiceColor::try_from('K'), Ok(DiceColor::Black));
        assert_eq!(DiceColor::try_from('R'), Ok(DiceColor::Red));
        assert_eq!(DiceColor::try_from('G'), Ok(DiceColor::Green));
        assert_eq!(DiceColor::try_from('B'), Ok(DiceColor::Blue));
        assert_eq!(DiceColor::try_from('Y'), Ok(DiceColor::Yellow));
        assert_eq!(DiceColor::try_from('O'), Ok(DiceColor::Orange));
    }

    #[test]
    fn try_from_char_case_insensitive() {
        assert_eq!(DiceColor::try_from('k'), Ok(DiceColor::Black));
        assert_eq!(DiceColor::try_from('r'), Ok(DiceColor::Red));
        assert_eq!(DiceColor::try_from('o'), Ok(DiceColor::Orange));
    }

    #[test]
    fn try_from_char_invalid() {
        assert!(DiceColor::try_from('X').is_err());
        assert!(DiceColor::try_from('1').is_err());
    }
}
