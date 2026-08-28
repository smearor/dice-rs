use dice_rs::error::Result;
use dice_rs::model::dice::DiceColor;
use dice_rs::model::dice::DiceType;
use dice_rs::model::led::LedColor;
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use dice_rs::service::dice::Dice;
use tracing::debug;

use crate::config::config_dir;
use crate::config::dice_settings::DiceSettings;

/// Service for pro-dice business operations.
///
/// Encapsulates dice hardware commands and config persistence,
/// keeping UI widgets free of business logic. All fire-and-forget
/// methods spawn async tasks internally so callers never block.
#[derive(Clone)]
pub struct DiceService {
    dice: Dice,
    device_name: String,
}

impl DiceService {
    /// Create a new dice service, loading persisted settings and applying
    /// the saved dice type to the hardware model.
    pub fn new(dice: Dice) -> Self {
        let device_name = dice.name().to_string();
        let service = Self { dice, device_name };
        let settings = service.load_settings();
        service.dice.set_dice_type(settings.dice_type);
        service
    }

    /// Returns a reference to the underlying dice.
    pub fn dice(&self) -> &Dice {
        &self.dice
    }

    // --- Config ---

    /// Load per-dice settings from disk.
    pub fn load_settings(&self) -> DiceSettings {
        config_dir::load_dice_settings(&self.device_name).unwrap_or_default()
    }

    /// Save per-dice settings to disk.
    pub fn save_settings(&self, settings: &DiceSettings) {
        config_dir::save_dice_settings(&self.device_name, settings);
    }

    // --- Dice Type ---

    /// Set the dice type on the hardware model and persist to config.
    pub fn set_dice_type(&self, dice_type: DiceType) {
        self.dice.set_dice_type(dice_type);
        let mut settings = self.load_settings();
        settings.dice_type = dice_type;
        self.save_settings(&settings);
    }

    // --- LED ---

    /// Set LED 1 color, persist to config, and apply to hardware.
    pub fn set_led1(&self, color: LedColor) {
        let mut settings = self.load_settings();
        settings.led_color1 = color;
        self.save_settings(&settings);
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.set_leds_immediate(color, LedColor::OFF).await {
                debug!(error = %error, "failed to set LED 1");
            }
        });
    }

    /// Set LED 2 color, persist to config, and apply to hardware.
    pub fn set_led2(&self, color: LedColor) {
        let mut settings = self.load_settings();
        settings.led_color2 = color;
        self.save_settings(&settings);
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.set_leds_immediate(LedColor::OFF, color).await {
                debug!(error = %error, "failed to set LED 2");
            }
        });
    }

    /// Set both LEDs, persist to config, and apply to hardware.
    pub fn set_leds(&self, color1: LedColor, color2: LedColor) {
        let mut settings = self.load_settings();
        settings.led_color1 = color1;
        settings.led_color2 = color2;
        self.save_settings(&settings);
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.set_leds_immediate(color1, color2).await {
                debug!(error = %error, "failed to set LEDs");
            }
        });
    }

    /// Turn off both LEDs and persist to config.
    pub fn turn_off_leds(&self) {
        let mut settings = self.load_settings();
        settings.led_color1 = LedColor::OFF;
        settings.led_color2 = LedColor::OFF;
        self.save_settings(&settings);
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.set_leds_immediate(LedColor::OFF, LedColor::OFF).await {
                debug!(error = %error, "failed to turn off LEDs");
            }
        });
    }

    /// Pulse LEDs with the given parameters (transient, not persisted).
    pub fn pulse_leds(&self, pulse_count: u8, on_time: u8, off_time: u8, color: LedColor, blink_mode: PulseBlinkMode, leds: PulseLeds) {
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.pulse_leds(pulse_count, on_time, off_time, color, blink_mode, leds).await {
                debug!(error = %error, "failed to pulse LEDs");
            }
        });
    }

    // --- Tap ---

    /// Enable single tap interrupt notifications.
    pub fn enable_tap(&self) {
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.enable_tap().await {
                debug!(error = %error, "failed to enable tap");
            }
        });
    }

    /// Disable single tap interrupt notifications.
    pub fn disable_tap(&self) {
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.disable_tap().await {
                debug!(error = %error, "failed to disable tap");
            }
        });
    }

    /// Enable double tap interrupt notifications.
    pub fn enable_double_tap(&self) {
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.enable_double_tap().await {
                debug!(error = %error, "failed to enable double tap");
            }
        });
    }

    /// Disable double tap interrupt notifications.
    pub fn disable_double_tap(&self) {
        let dice = self.dice.clone();
        tokio::spawn(async move {
            if let Err(error) = dice.disable_double_tap().await {
                debug!(error = %error, "failed to disable double tap");
            }
        });
    }

    // --- Async queries (caller awaits) ---

    /// Request the dice physical color.
    pub async fn get_color(&self) -> Result<DiceColor> {
        self.dice.get_color().await
    }
}
