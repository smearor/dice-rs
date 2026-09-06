use dice_rs::LedColor;
use serde::Deserialize;
use serde::Serialize;

/// A player's color used for LED indication and UI display.
///
/// Wraps `dice_rs::LedColor` to provide a semantic type specifically
/// for player identification. The color is set programmatically on
/// the physical GoDice LEDs, independent of the dice's physical color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerColor(LedColor);

impl PlayerColor {
    /// Create a player color from an `LedColor`.
    pub const fn new(color: LedColor) -> Self {
        Self(color)
    }

    /// Get the underlying `LedColor` for BLE commands.
    pub fn led_color(self) -> LedColor {
        self.0
    }

    /// Red player color.
    pub const RED: Self = Self::new(LedColor::RED);

    /// Green player color.
    pub const GREEN: Self = Self::new(LedColor::GREEN);

    /// Blue player color.
    pub const BLUE: Self = Self::new(LedColor::BLUE);

    /// Yellow player color.
    pub const YELLOW: Self = Self::new(LedColor::new(255, 255, 0));

    /// Orange player color.
    pub const ORANGE: Self = Self::new(LedColor::new(255, 165, 0));

    /// Purple player color.
    pub const PURPLE: Self = Self::new(LedColor::new(128, 0, 255));

    /// Cyan player color.
    pub const CYAN: Self = Self::new(LedColor::new(0, 255, 255));

    /// White player color.
    pub const WHITE: Self = Self::new(LedColor::WHITE);

    /// Returns a list of distinct default player colors.
    pub const DEFAULTS: [Self; 6] = [Self::RED, Self::GREEN, Self::BLUE, Self::YELLOW, Self::ORANGE, Self::PURPLE];
}

impl From<LedColor> for PlayerColor {
    fn from(color: LedColor) -> Self {
        Self(color)
    }
}

impl std::fmt::Display for PlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_led_color() {
        let color = PlayerColor::new(LedColor::RED);
        assert_eq!(color.led_color(), LedColor::RED);
    }

    #[test]
    fn defaults_has_6_colors() {
        assert_eq!(PlayerColor::DEFAULTS.len(), 6);
    }

    #[test]
    fn all_defaults_are_distinct() {
        for i in 0..PlayerColor::DEFAULTS.len() {
            for j in (i + 1)..PlayerColor::DEFAULTS.len() {
                assert_ne!(PlayerColor::DEFAULTS[i], PlayerColor::DEFAULTS[j]);
            }
        }
    }

    #[test]
    fn from_led_color() {
        let color: PlayerColor = LedColor::GREEN.into();
        assert_eq!(color.led_color(), LedColor::GREEN);
    }
}
