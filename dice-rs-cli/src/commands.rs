use std::time::Duration;

use dice_rs::model::led::LedColor;
use dice_rs::service::dice::Dice;
use dice_rs::service::dice::DiceDevice;
use dice_rs::service::dice::DiceEvent;
use dice_rs::service::manager::DiceManager;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::sync::broadcast;
use tracing::debug;

use crate::cli_error::CliError;
use crate::cli_error::Result;
use crate::command::Command;
use crate::led_action::LedAction;
use crate::output;
use crate::output_format::OutputFormat;

/// Find a device by MAC address from scan results.
async fn find_device_by_address(manager: &DiceManager, address: &str) -> Result<DiceDevice> {
    let devices = manager.scan().await?;
    devices
        .into_iter()
        .find(|d| d.address.to_string().contains(address))
        .ok_or_else(|| CliError::DeviceNotFound(address.to_string()))
}

/// Connect to a dice by address (scans first if needed).
async fn connect_by_address(manager: &DiceManager, address: &str) -> Result<Dice> {
    let device = find_device_by_address(manager, address).await?;
    let dice = manager.connect(&device).await?;
    Ok(dice)
}

/// Run a subcommand.
pub async fn run_command(manager: &DiceManager, command: Command, format: OutputFormat) -> Result<()> {
    match command {
        Command::Scan { duration } => run_scan(manager, duration, format).await,
        Command::Listen { address, dice_type } => run_listen(manager, &address, &dice_type, format).await,
        Command::Battery { address } => run_battery(manager, &address, format).await,
        Command::Led { address, action } => run_led(manager, &address, action).await,
        Command::Calibrate { address } => run_calibrate(manager, &address).await,
        Command::Status { address } => run_status(manager, &address, format).await,
        Command::Color { address } => run_color(manager, &address, format).await,
        Command::Charging { address } => run_charging(manager, &address, format).await,
        Command::Interactive => run_interactive(manager).await,
    }
}

async fn run_scan(manager: &DiceManager, duration: u64, format: OutputFormat) -> Result<()> {
    let scanner = manager.scanner().with_scan_duration(Duration::from_secs(duration));
    let devices = scanner.scan().await?;
    output::print_devices(&devices, format);
    Ok(())
}

async fn run_listen(manager: &DiceManager, address: &str, dice_type: &str, format: OutputFormat) -> Result<()> {
    let device = find_device_by_address(manager, address).await?;
    let dice = manager.connect(&device).await?;
    let dt = output::parse_dice_type(dice_type)?;
    dice.set_dice_type(dt);

    let mut events = dice.subscribe();
    println!("Listening for events from {address} (Ctrl+C to stop)...");

    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    Ok(event) => {
                        let message = event.to_string();
                        output::print_event(&message, format);
                        if matches!(event, DiceEvent::Disconnected) {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        debug!("missed {n} events");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nStopping...");
                break;
            }
        }
    }

    dice.disconnect().await?;
    Ok(())
}

async fn run_led(manager: &DiceManager, address: &str, action: LedAction) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;

    match action {
        LedAction::Set { color } => {
            let color = output::parse_color(&color)?;
            dice.set_leds_immediate(color, color).await?;
            println!("LEDs set to {color}");
        }
        LedAction::SetDual { led1, led2 } => {
            let led1 = output::parse_color(&led1)?;
            let led2 = output::parse_color(&led2)?;
            dice.set_leds_immediate(led1, led2).await?;
            println!("LED 1: {led1}, LED 2: {led2}");
        }
        LedAction::Pulse {
            color,
            count,
            on_time,
            off_time,
        } => {
            let color = output::parse_color(&color)?;
            dice.pulse_leds(count, on_time, off_time, color).await?;
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

async fn run_battery(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;
    let level = dice.get_battery_level().await?;
    output::print_battery(&level, format);
    dice.disconnect().await?;
    Ok(())
}

async fn run_calibrate(manager: &DiceManager, address: &str) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;
    println!("Place the dice on a flat surface and press Enter to calibrate...");
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut input = String::new();
    reader.read_line(&mut input).await?;
    dice.calibrate().await?;
    println!("Calibration complete.");
    dice.disconnect().await?;
    Ok(())
}

async fn run_status(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;
    let status = dice.system_status().await?;
    output::print_status(&status, format);
    dice.disconnect().await?;
    Ok(())
}

async fn run_color(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;
    let color = dice.get_color().await?;
    output::print_color(color, format);
    dice.disconnect().await?;
    Ok(())
}

async fn run_charging(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = connect_by_address(manager, address).await?;
    let charging = dice.is_charging();
    output::print_charging(charging, format);
    dice.disconnect().await?;
    Ok(())
}

async fn run_interactive(manager: &DiceManager) -> Result<()> {
    use std::io::Write;

    println!("dice-rs interactive mode. Type 'help' for commands, 'quit' to exit.");

    let mut dice: Option<Dice> = None;
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();
        print!("dice-rs> ");
        std::io::stdout().flush().ok();
        reader.read_line(&mut input).await?;

        let input = input.trim();
        match input {
            "help" => print_interactive_help(),
            "quit" | "exit" => break,
            "scan" => {
                let devices = manager.scan().await?;
                output::print_devices(&devices, OutputFormat::Table);
            }
            cmd if cmd.starts_with("connect ") => {
                let address = cmd.strip_prefix("connect ").unwrap_or(cmd);
                let device = find_device_by_address(manager, address).await?;
                dice = Some(manager.connect(&device).await?);
                println!("Connected to {address}");
            }
            "disconnect" => {
                if let Some(d) = dice.take() {
                    d.disconnect().await?;
                    println!("Disconnected");
                }
            }
            "battery" => {
                if let Some(d) = &dice {
                    let level = d.get_battery_level().await?;
                    println!("Battery: {level}");
                }
            }
            "color" => {
                if let Some(d) = &dice {
                    let color = d.get_color().await?;
                    println!("Color: {color}");
                }
            }
            "charging" => {
                if let Some(d) = &dice {
                    let charging = d.is_charging();
                    println!("Charging: {charging}");
                }
            }
            cmd if cmd.starts_with("led ") => {
                if let Some(d) = &dice {
                    let color_str = cmd.strip_prefix("led ").unwrap_or(cmd);
                    let color = output::parse_color(color_str)?;
                    d.set_leds_immediate(color, color).await?;
                    println!("LEDs set to {color}");
                }
            }
            "status" => {
                if let Some(d) = &dice {
                    let status = d.system_status().await?;
                    output::print_status(&status, OutputFormat::Table);
                }
            }
            "calibrate" => {
                if let Some(d) = &dice {
                    d.calibrate().await?;
                    println!("Calibration complete");
                }
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }

    if let Some(d) = dice.take() {
        d.disconnect().await?;
    }
    Ok(())
}

fn print_interactive_help() {
    println!("Available commands:");
    println!("  scan              Scan for GoDice devices");
    println!("  connect <addr>    Connect to a device by MAC address");
    println!("  disconnect        Disconnect from the current device");
    println!("  battery           Query battery level");
    println!("  color             Query dice color");
    println!("  charging          Check if dice is charging");
    println!("  led <color>       Set LEDs to a color (named or hex)");
    println!("  status            Show system status");
    println!("  calibrate         Calibrate the dice");
    println!("  help              Show this help");
    println!("  quit              Exit interactive mode");
}
