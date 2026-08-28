use crate::model::dice::transforms::D4_TRANSFORM;
use crate::model::dice::transforms::D8_TRANSFORM;
use crate::model::dice::transforms::D10_TRANSFORM;
use crate::model::dice::transforms::D10X_TRANSFORM;
use crate::model::dice::transforms::D12_TRANSFORM;
use crate::model::dice::type_error::DiceTypeError;
use crate::model::dice::vectors::D6_VECTORS;
use crate::model::dice::vectors::D20_VECTORS;
use crate::model::dice::vectors::D24_VECTORS;
use std::str::FromStr;

/// Determines which vector table and shell transform are used to interpret
/// accelerometer data into a face value.
///
/// `#[repr(u8)]` allows storage as `AtomicU8` for lock-free reads
/// in the notification task hot path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DiceType {
    /// Standard 6-sided die (default).
    #[default]
    D6 = 0,
    /// 20-sided die.
    D20 = 1,
    /// 10-sided die (values 1–10).
    D10 = 2,
    /// 10-sided "tens" die (values 00, 10, 20, ..., 90).
    D10X = 3,
    /// 4-sided die.
    D4 = 4,
    /// 8-sided die.
    D8 = 5,
    /// 12-sided die.
    D12 = 6,
}

impl From<DiceType> for u8 {
    fn from(dt: DiceType) -> Self {
        dt as u8
    }
}

impl TryFrom<u8> for DiceType {
    type Error = DiceTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::D6),
            1 => Ok(Self::D20),
            2 => Ok(Self::D10),
            3 => Ok(Self::D10X),
            4 => Ok(Self::D4),
            5 => Ok(Self::D8),
            6 => Ok(Self::D12),
            _ => Err(DiceTypeError::InvalidValue(value)),
        }
    }
}

impl std::fmt::Display for DiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D6 => write!(f, "D6"),
            Self::D20 => write!(f, "D20"),
            Self::D10 => write!(f, "D10"),
            Self::D10X => write!(f, "D10X"),
            Self::D4 => write!(f, "D4"),
            Self::D8 => write!(f, "D8"),
            Self::D12 => write!(f, "D12"),
        }
    }
}

/// Parse a dice type from a string.
///
/// Accepts case-insensitive names: `"d6"`, `"d20"`, `"d10"`, `"d10x"`, `"d4"`, `"d8"`, `"d12"`.
///
/// ```
/// use std::str::FromStr;
/// use dice_rs::model::dice::DiceType;
/// let dt = DiceType::from_str("d20").unwrap();
/// assert_eq!(dt, DiceType::D20);
/// ```
impl FromStr for DiceType {
    type Err = DiceTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "d6" => Ok(Self::D6),
            "d20" => Ok(Self::D20),
            "d10" => Ok(Self::D10),
            "d10x" => Ok(Self::D10X),
            "d4" => Ok(Self::D4),
            "d8" => Ok(Self::D8),
            "d12" => Ok(Self::D12),
            _ => Err(DiceTypeError::InvalidName(input.to_string())),
        }
    }
}

impl DiceType {
    /// Returns the vector table used for this dice type.
    pub fn vector_table(&self) -> &'static [(i32, i32, i32)] {
        match self {
            Self::D6 => &D6_VECTORS,
            Self::D20 | Self::D10 | Self::D10X => &D20_VECTORS,
            Self::D4 | Self::D8 | Self::D12 => &D24_VECTORS,
        }
    }

    /// Returns the shell transform table for this dice type, if any.
    pub fn transform(&self) -> Option<&'static [u8]> {
        match self {
            Self::D6 | Self::D20 => None,
            Self::D10 => Some(&D10_TRANSFORM),
            Self::D10X => Some(&D10X_TRANSFORM),
            Self::D4 => Some(&D4_TRANSFORM),
            Self::D8 => Some(&D8_TRANSFORM),
            Self::D12 => Some(&D12_TRANSFORM),
        }
    }

    pub fn sorted_by_count() -> Vec<DiceType> {
        vec![
            DiceType::D4,
            DiceType::D6,
            DiceType::D8,
            DiceType::D10,
            DiceType::D10X,
            DiceType::D12,
            DiceType::D20
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid() {
        assert_eq!(DiceType::try_from(0), Ok(DiceType::D6));
        assert_eq!(DiceType::try_from(6), Ok(DiceType::D12));
    }

    #[test]
    fn try_from_invalid() {
        assert!(DiceType::try_from(7).is_err());
    }

    #[test]
    fn from_to_u8_roundtrip() {
        for value in 0..=6 {
            let dice_type = DiceType::try_from(value).expect("valid dice type");
            assert_eq!(u8::from(dice_type), value);
        }
    }

    #[test]
    fn vector_table_d6() {
        assert_eq!(DiceType::D6.vector_table().len(), 6);
    }

    #[test]
    fn vector_table_d20() {
        assert_eq!(DiceType::D20.vector_table().len(), 20);
    }

    #[test]
    fn vector_table_d24() {
        assert_eq!(DiceType::D4.vector_table().len(), 24);
    }

    #[test]
    fn transform_none_for_d6_d20() {
        assert!(DiceType::D6.transform().is_none());
        assert!(DiceType::D20.transform().is_none());
    }

    #[test]
    fn transform_some_for_shells() {
        assert!(DiceType::D10.transform().is_some());
        assert!(DiceType::D10X.transform().is_some());
        assert!(DiceType::D4.transform().is_some());
        assert!(DiceType::D8.transform().is_some());
        assert!(DiceType::D12.transform().is_some());
    }

    #[test]
    fn display() {
        assert_eq!(DiceType::D6.to_string(), "D6");
        assert_eq!(DiceType::D10X.to_string(), "D10X");
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(DiceType::from_str("d6").unwrap(), DiceType::D6);
        assert_eq!(DiceType::from_str("D20").unwrap(), DiceType::D20);
        assert_eq!(DiceType::from_str("d10x").unwrap(), DiceType::D10X);
        assert_eq!(DiceType::from_str("D4").unwrap(), DiceType::D4);
    }

    #[test]
    fn from_str_invalid_returns_err() {
        assert!(DiceType::from_str("d100").is_err());
        assert!(DiceType::from_str("xyz").is_err());
    }
}
