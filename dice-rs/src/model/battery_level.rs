use serde::Deserialize;
use serde::Serialize;

/// Battery level of a GoDice device (0–100 percent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BatteryLevel(u8);

impl BatteryLevel {
    /// Create a `BatteryLevel` from a raw byte.
    ///
    /// Values above 100 are clamped to 100.
    pub const fn new(level: u8) -> Self {
        Self(if level > 100 { 100 } else { level })
    }

    /// Returns the raw battery percentage (0–100).
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl From<u8> for BatteryLevel {
    fn from(level: u8) -> Self {
        Self::new(level)
    }
}

impl From<BatteryLevel> for u8 {
    fn from(level: BatteryLevel) -> Self {
        level.0
    }
}

impl std::fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        assert_eq!(BatteryLevel::new(0).get(), 0);
        assert_eq!(BatteryLevel::new(50).get(), 50);
        assert_eq!(BatteryLevel::new(100).get(), 100);
    }

    #[test]
    fn new_clamps_above_100() {
        assert_eq!(BatteryLevel::new(150).get(), 100);
        assert_eq!(BatteryLevel::new(255).get(), 100);
    }

    #[test]
    fn from_u8() {
        let level: BatteryLevel = 75u8.into();
        assert_eq!(level.get(), 75);
    }

    #[test]
    fn into_u8() {
        let level = BatteryLevel::new(42);
        let raw: u8 = level.into();
        assert_eq!(raw, 42);
    }

    #[test]
    fn display() {
        assert_eq!(BatteryLevel::new(75).to_string(), "75%");
        assert_eq!(BatteryLevel::new(0).to_string(), "0%");
    }
}
