use crate::error::DiceError;
use serde::Deserialize;
use serde::Serialize;

/// The face value rolled on a die (1-based).
///
/// A newtype wrapper around `u8` that enforces the invariant
/// that a face value is always ≥ 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FaceValue(u8);

impl FaceValue {
    /// The minimum valid face value (1).
    pub const ONE: Self = Self(1);

    /// Create a face value. Returns error if value is 0.
    pub fn new(value: u8) -> Result<Self, DiceError> {
        if value == 0 {
            return Err(DiceError::InvalidFaceValue(0));
        }
        Ok(Self(value))
    }

    /// Get the numeric value.
    pub fn get(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for FaceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(FaceValue::new(1).map(FaceValue::get), Ok(1));
        assert_eq!(FaceValue::new(20).map(FaceValue::get), Ok(20));
    }

    #[test]
    fn new_zero_rejected() {
        assert!(FaceValue::new(0).is_err());
    }

    #[test]
    fn display() {
        assert_eq!(FaceValue::new(6).unwrap().to_string(), "6");
    }
}
