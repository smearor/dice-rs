use crate::model::dice::color_error::DiceColorError;

/// Physical color of a GoDice device.
///
/// Encoded as a single byte in the `Col` response notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}
