use serde::Deserialize;
use serde::Serialize;

use dice_rs::model::dice::DiceType;
use dice_rs::model::led::LedColor;

/// Per-dice settings persisted to `~/.config/dice-rs/<device_name>.toml`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiceSettings {
    /// Dice shell type for face value interpretation.
    pub dice_type: DiceType,
    /// Color for LED 1.
    pub led_color1: LedColor,
    /// Color for LED 2.
    pub led_color2: LedColor,
}

impl Default for DiceSettings {
    fn default() -> Self {
        Self {
            dice_type: DiceType::D6,
            led_color1: LedColor::OFF,
            led_color2: LedColor::OFF,
        }
    }
}
