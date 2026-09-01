use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use crate::models::player_color::PlayerColor;
#[cfg(test)]
use dice_rs::LedColor;
#[cfg(test)]
use dice_rs::model::led::PulseBlinkMode;
#[cfg(test)]
use dice_rs::model::led::PulseLeds;

/// A logical LED effect to apply to the physical dice.
///
/// Effects are translated to BLE commands by `LedService`.
/// This abstraction allows the game logic to request effects
/// without knowing BLE protocol details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedEffect {
    /// Set all dice to a solid color (e.g. active player color).
    SolidAll {
        /// The color to set on all dice.
        color: PlayerColor,
    },
    /// Set a single die to a specific color.
    Solid {
        /// The slot of the die to update.
        slot: DiceSlot,
        /// The color to set.
        color: PlayerColor,
    },
    /// Turn off all dice LEDs.
    OffAll,
    /// Turn off a single die's LEDs.
    Off {
        /// The slot of the die to turn off.
        slot: DiceSlot,
    },
    /// Visualize held dice: held dice glow green, others are off.
    HoldVisualization {
        /// Which dice are held.
        holds: HoldMask,
    },
    /// Visualize held dice with a custom color.
    HoldVisualizationWithColor {
        /// Which dice are held.
        holds: HoldMask,
        /// Color for held dice.
        held_color: PlayerColor,
    },
    /// Pulse all dice with a celebration color (e.g. Yahtzee, Full House).
    Celebrate {
        /// Number of pulses.
        pulse_count: u8,
        /// Color to pulse.
        color: PlayerColor,
    },
    /// Pulse a single die.
    Pulse {
        /// The slot of the die to pulse.
        slot: DiceSlot,
        /// Number of pulses.
        pulse_count: u8,
        /// Color to pulse.
        color: PlayerColor,
    },
    /// Rainbow pulse on all dice (for special events).
    Rainbow {
        /// Number of pulses.
        pulse_count: u8,
    },
}

impl LedEffect {
    /// Celebration effect for rolling a Yahtzee (5 of a kind).
    pub fn yahtzee() -> Self {
        Self::Celebrate {
            pulse_count: 5,
            color: PlayerColor::GREEN,
        }
    }

    /// Celebration effect for rolling a Full House.
    pub fn full_house() -> Self {
        Self::Celebrate {
            pulse_count: 3,
            color: PlayerColor::YELLOW,
        }
    }

    /// Celebration effect for rolling a Large Straight.
    pub fn large_straight() -> Self {
        Self::Celebrate {
            pulse_count: 3,
            color: PlayerColor::CYAN,
        }
    }

    /// Active player indicator: all dice glow in the player's color.
    pub fn active_player(color: PlayerColor) -> Self {
        Self::SolidAll { color }
    }

    /// Turn all dice off.
    pub fn all_off() -> Self {
        Self::OffAll
    }

    /// Convert a `PlayerColor` to `LedColor`.
    #[cfg(test)]
    pub(crate) fn led_color(color: PlayerColor) -> LedColor {
        color.led_color()
    }

    /// Get the pulse parameters for this effect, if it is a pulse effect.
    #[cfg(test)]
    pub(crate) fn pulse_params(&self) -> Option<(u8, LedColor, PulseBlinkMode, PulseLeds)> {
        match self {
            Self::Celebrate { pulse_count, color } => Some((*pulse_count, Self::led_color(*color), PulseBlinkMode::Color, PulseLeds::Both)),
            Self::Rainbow { pulse_count } => Some((*pulse_count, LedColor::WHITE, PulseBlinkMode::Rainbow, PulseLeds::Both)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yahtzee_effect() {
        let effect = LedEffect::yahtzee();
        assert_eq!(
            effect,
            LedEffect::Celebrate {
                pulse_count: 5,
                color: PlayerColor::GREEN
            }
        );
    }

    #[test]
    fn full_house_effect() {
        let effect = LedEffect::full_house();
        assert_eq!(
            effect,
            LedEffect::Celebrate {
                pulse_count: 3,
                color: PlayerColor::YELLOW
            }
        );
    }

    #[test]
    fn active_player_effect() {
        let effect = LedEffect::active_player(PlayerColor::RED);
        assert_eq!(effect, LedEffect::SolidAll { color: PlayerColor::RED });
    }

    #[test]
    fn pulse_params_for_celebrate() {
        let effect = LedEffect::yahtzee();
        let params = effect.pulse_params();
        assert!(params.is_some());
        let (count, color, mode, leds) = params.unwrap();
        assert_eq!(count, 5);
        assert_eq!(color, LedColor::GREEN);
        assert_eq!(mode, PulseBlinkMode::Color);
        assert_eq!(leds, PulseLeds::Both);
    }

    #[test]
    fn pulse_params_for_rainbow() {
        let effect = LedEffect::Rainbow { pulse_count: 3 };
        let params = effect.pulse_params();
        assert!(params.is_some());
        let (_, _, mode, _) = params.unwrap();
        assert_eq!(mode, PulseBlinkMode::Rainbow);
    }

    #[test]
    fn pulse_params_none_for_solid() {
        let effect = LedEffect::SolidAll { color: PlayerColor::RED };
        assert!(effect.pulse_params().is_none());
    }

    #[test]
    fn all_off() {
        assert_eq!(LedEffect::all_off(), LedEffect::OffAll);
    }
}
