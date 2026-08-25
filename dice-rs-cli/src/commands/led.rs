use std::str::FromStr;

use dice_rs::model::led::LedColor;
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use dice_rs::service::manager::DiceManager;

use crate::cli_error::CliError;
use crate::cli_error::Result;
use crate::led_action::LedAction;

pub async fn run(manager: &DiceManager, address: &str, action: LedAction) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;

    match action {
        LedAction::Set { color } => {
            let color = LedColor::from_str(&color)?;
            dice.set_leds_immediate(color, color).await?;
            println!("LEDs set to {color}");
        }
        LedAction::SetDual { led1, led2 } => {
            let led1 = LedColor::from_str(&led1)?;
            let led2 = LedColor::from_str(&led2)?;
            dice.set_leds_immediate(led1, led2).await?;
            println!("LED 1: {led1}, LED 2: {led2}");
        }
        LedAction::Pulse {
            color,
            count,
            on_time,
            off_time,
            blink_mode,
            leds,
        } => {
            let color = LedColor::from_str(&color)?;
            let blink_mode: PulseBlinkMode = blink_mode.parse().map_err(CliError::from)?;
            let leds: PulseLeds = leds.parse().map_err(CliError::from)?;
            dice.pulse_leds(count, on_time, off_time, color, blink_mode, leds).await?;
            println!("Pulsing {color} x{count} ({on_time}0ms on / {off_time}0ms off)");
        }
        LedAction::Off => {
            dice.set_leds_immediate(LedColor::OFF, LedColor::OFF).await?;
            println!("LEDs off");
        }
    }

    dice.disconnect().await?;
    Ok(())
}
