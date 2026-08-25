use std::io::Write;
use std::str::FromStr;

use dice_rs::model::led::LedColor;
use dice_rs::service::dice::Dice;
use dice_rs::service::manager::DiceManager;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

use crate::cli_error::Result;
use crate::commands::repl_command::ReplCommand;
use crate::output;
use crate::output::ScanResults;
use crate::output_format::OutputFormat;

pub async fn run(manager: &DiceManager) -> Result<()> {
    println!("dice-rs interactive mode. Type 'help' for commands, 'quit' to exit.");

    let mut dice: Option<Dice> = None;
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();
        print!("dice-rs> ");
        std::io::stdout().flush().ok();
        reader.read_line(&mut input).await?;

        let command = match ReplCommand::from_str(&input) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match command {
            ReplCommand::Help => ReplCommand::print_help(),
            ReplCommand::Quit => break,
            ReplCommand::Scan => {
                let devices = manager.scan().await?;
                output::print(&ScanResults::from(devices), OutputFormat::Table);
            }
            ReplCommand::Connect(address) => {
                let device = manager.find_device_by_address(&address).await?;
                dice = Some(manager.connect(&device).await?);
                println!("Connected to {address}");
            }
            ReplCommand::Disconnect => {
                if let Some(d) = dice.take() {
                    d.disconnect().await?;
                    println!("Disconnected");
                }
            }
            ReplCommand::Battery => {
                if let Some(d) = &dice {
                    let level = d.get_battery_level().await?;
                    println!("Battery: {level}");
                }
            }
            ReplCommand::Color => {
                if let Some(d) = &dice {
                    let color = d.get_color().await?;
                    println!("Color: {color}");
                }
            }
            ReplCommand::Charging => {
                if let Some(d) = &dice {
                    let charging = d.charging_state();
                    println!("Charging: {charging}");
                }
            }
            ReplCommand::Led(color_str) => {
                if let Some(d) = &dice {
                    let color = LedColor::from_str(&color_str)?;
                    d.set_leds_immediate(color, color).await?;
                    println!("LEDs set to {color}");
                }
            }
            ReplCommand::Status => {
                if let Some(d) = &dice {
                    let status = d.system_status().await?;
                    output::print(&status, OutputFormat::Table);
                }
            }
            ReplCommand::Calibrate => {
                if let Some(d) = &dice {
                    d.calibrate().await?;
                    println!("Calibration complete");
                }
            }
        }
    }

    if let Some(d) = dice.take() {
        d.disconnect().await?;
    }
    Ok(())
}
