use crate::error::Result;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use crate::models::player_color::PlayerColor;
use crate::services::led_effect::LedEffect;
use crate::services::slot_mapping::SlotMapping;
use dice_rs::LedColor;
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use tracing::debug;

/// Service for controlling GoDice LED effects.
///
/// Translates logical `LedEffect`s into BLE commands sent to the
/// physical dice. Uses `SlotMapping` to find the correct `Dice` handle
/// for each slot.
#[derive(Clone)]
pub struct LedService {
    /// Mapping from game slots to physical dice.
    mapping: SlotMapping,
}

impl LedService {
    /// Create a new LED service with the given slot mapping.
    pub fn new(mapping: SlotMapping) -> Self {
        Self { mapping }
    }

    /// Apply an LED effect to the physical dice.
    ///
    /// Returns `Ok(())` if the effect was applied to at least one die.
    /// Errors from individual dice are logged and do not abort the
    /// operation — partial LED failures are non-fatal.
    pub async fn apply(&self, effect: &LedEffect) -> Result<()> {
        match effect {
            LedEffect::SolidAll { color } => {
                self.set_all_leds(color.led_color()).await;
            }
            LedEffect::Solid { slot, color } => {
                self.set_slot_led(*slot, color.led_color()).await;
            }
            LedEffect::OffAll => {
                self.set_all_leds(LedColor::OFF).await;
            }
            LedEffect::Off { slot } => {
                self.set_slot_led(*slot, LedColor::OFF).await;
            }
            LedEffect::HoldVisualization { holds } => {
                self.apply_hold_visualization(holds, PlayerColor::GREEN).await;
            }
            LedEffect::HoldVisualizationWithColor { holds, held_color } => {
                self.apply_hold_visualization(holds, *held_color).await;
            }
            LedEffect::Celebrate { pulse_count, color } => {
                self.pulse_all(*pulse_count, color.led_color()).await;
            }
            LedEffect::Pulse { slot, pulse_count, color } => {
                self.pulse_slot(*slot, *pulse_count, color.led_color()).await;
            }
            LedEffect::Rainbow { pulse_count } => {
                self.rainbow_all(*pulse_count).await;
            }
        }
        Ok(())
    }

    /// Set all connected dice to a solid color.
    async fn set_all_leds(&self, color: LedColor) {
        for (slot, dice) in self.mapping.assigned() {
            if let Err(error) = dice.set_led(color).await {
                debug!(slot = %slot.get(), error = %error, "failed to set LED on dice");
            }
        }
    }

    /// Set a single die's LED to a solid color.
    async fn set_slot_led(&self, slot: DiceSlot, color: LedColor) {
        if let Some(dice) = self.mapping.get(slot)
            && let Err(error) = dice.set_led(color).await
        {
            debug!(slot = %slot.get(), error = %error, "failed to set LED on dice");
        }
    }

    /// Apply hold visualization: held dice glow, others off.
    async fn apply_hold_visualization(&self, holds: &HoldMask, held_color: PlayerColor) {
        for slot in DiceSlot::all() {
            let color = if holds.is_held(slot) { held_color.led_color() } else { LedColor::OFF };
            if let Some(dice) = self.mapping.get(slot)
                && let Err(error) = dice.set_led(color).await
            {
                debug!(slot = %slot.get(), error = %error, "failed to set LED for hold visualization");
            }
        }
    }

    /// Pulse all connected dice with a color.
    async fn pulse_all(&self, _pulse_count: u8, color: LedColor) {
        for (slot, dice) in self.mapping.assigned() {
            if let Err(error) = dice.pulse_once(50, 50, color).await {
                debug!(slot = %slot.get(), error = %error, "failed to pulse dice");
            }
        }
    }

    /// Pulse a single die with a color.
    async fn pulse_slot(&self, slot: DiceSlot, pulse_count: u8, color: LedColor) {
        if let Some(dice) = self.mapping.get(slot)
            && let Err(error) = dice.pulse_once(pulse_count, 50, color).await
        {
            debug!(slot = %slot.get(), error = %error, "failed to pulse dice");
        }
    }

    /// Rainbow pulse on all connected dice.
    async fn rainbow_all(&self, pulse_count: u8) {
        for (slot, dice) in self.mapping.assigned() {
            if let Err(error) = dice
                .pulse_leds(pulse_count, 50, 50, LedColor::WHITE, PulseBlinkMode::Rainbow, PulseLeds::Both)
                .await
            {
                debug!(slot = %slot.get(), error = %error, "failed to rainbow pulse dice");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_service() {
        let mapping = SlotMapping::new();
        let _service = LedService::new(mapping);
    }
}
