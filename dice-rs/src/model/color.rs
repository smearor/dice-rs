/// Physical color of a GoDice device.
///
/// Encoded as a single byte in the `Col` response notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DieColor {
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

impl TryFrom<u8> for DieColor {
    type Error = DieColorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::Red),
            2 => Ok(Self::Green),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Orange),
            _ => Err(DieColorError::InvalidValue(value)),
        }
    }
}

impl From<DieColor> for u8 {
    fn from(color: DieColor) -> Self {
        color as u8
    }
}

impl std::fmt::Display for DieColor {
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

/// Error returned when an invalid dice color byte is encountered.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DieColorError {
    /// The byte does not correspond to any known `DieColor`.
    #[error("invalid dice color value: {0}")]
    InvalidValue(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid() {
        assert_eq!(DieColor::try_from(0), Ok(DieColor::Black));
        assert_eq!(DieColor::try_from(5), Ok(DieColor::Orange));
    }

    #[test]
    fn try_from_invalid() {
        assert!(DieColor::try_from(6).is_err());
    }

    #[test]
    fn from_to_u8_roundtrip() {
        for value in 0..=5 {
            let color = DieColor::try_from(value).expect("valid color");
            assert_eq!(u8::from(color), value);
        }
    }

    #[test]
    fn display() {
        assert_eq!(DieColor::Red.to_string(), "Red");
    }
}
