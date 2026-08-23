/// An RGB color for a GoDice LED.
///
/// Each channel is in the range 0–255. `(0, 0, 0)` turns the LED off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LedColor {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
}

impl LedColor {
    /// Black (LED off).
    pub const OFF: Self = Self { r: 0, g: 0, b: 0 };

    /// Red.
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };

    /// Green.
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };

    /// Blue.
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    /// White.
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    /// Create a new color from RGB values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a color from a 24-bit hex value (e.g. `0xFF8800`).
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Convert to a 24-bit hex value.
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Returns true if all channels are zero (LED off).
    pub fn is_off(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl From<(u8, u8, u8)> for LedColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl std::fmt::Display for LedColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let color = LedColor::new(10, 20, 30);
        assert_eq!(color.r, 10);
        assert_eq!(color.g, 20);
        assert_eq!(color.b, 30);
    }

    #[test]
    fn from_hex() {
        let color = LedColor::from_hex(0xFF8800);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 136);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn to_hex() {
        let color = LedColor::new(255, 136, 0);
        assert_eq!(color.to_hex(), 0xFF8800);
    }

    #[test]
    fn is_off() {
        assert!(LedColor::OFF.is_off());
        assert!(!LedColor::RED.is_off());
    }

    #[test]
    fn from_tuple() {
        let color: LedColor = (1, 2, 3).into();
        assert_eq!(color, LedColor::new(1, 2, 3));
    }

    #[test]
    fn display() {
        assert_eq!(LedColor::RED.to_string(), "#FF0000");
    }
}
