use std::str::FromStr;

use dice_rs::model::dice::DiceColor;
use dice_rs::model::dice::DiceType;
use dice_rs::model::led::LedColor;
use dice_rs::model::system_status::SystemStatus;
use dice_rs::service::dice::DiceDevice;

use crate::cli_error::CliError;
use crate::cli_error::Result;
use crate::output_format::OutputFormat;

/// Print a timestamped event line.
pub fn print_event(message: &str, format: OutputFormat) {
    match format {
        OutputFormat::Table | OutputFormat::Plain => {
            let now = chrono_like_timestamp();
            println!("[{now}] {message}");
        }
        OutputFormat::Json => {
            let json = serde_json::json!({ "event": message });
            println!("{json}");
        }
    }
}

/// Print scan results in the selected format.
pub fn print_devices(devices: &[DiceDevice], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            let rows: Vec<crate::device_row::DeviceRow> = devices.iter().map(crate::device_row::DeviceRow::from).collect();
            if rows.is_empty() {
                println!("No GoDice devices found.");
                return;
            }
            let table = tabled::Table::new(&rows).with(tabled::settings::Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            let json: Vec<_> = devices
                .iter()
                .map(|d| {
                    serde_json::json!({
                        "address": d.address.to_string(),
                        "name": d.name,
                        "rssi": d.rssi,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string(&json).unwrap_or_else(|_| "[]".into()));
        }
        OutputFormat::Plain => {
            for d in devices {
                println!("{} {} {}", d.address, d.name, d.rssi.map(|r| format!("{r}")).unwrap_or_else(|| "N/A".into()));
            }
        }
    }
}

/// Print battery level in the selected format.
pub fn print_battery(level: &dice_rs::model::battery_level::BatteryLevel, format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            let row = crate::battery_row::BatteryRow { battery: format!("{level}") };
            let table = tabled::Table::new(vec![row]).with(tabled::settings::Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            println!(r#"{{"battery_level":{}}}"#, level.get());
        }
        OutputFormat::Plain => {
            println!("{level}");
        }
    }
}

/// Print system status in the selected format.
pub fn print_status(status: &SystemStatus, format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            let rows = vec![
                crate::status_row::StatusRow {
                    property: "Battery".into(),
                    value: format!("{}", status.battery_level),
                },
                crate::status_row::StatusRow {
                    property: "Color".into(),
                    value: format!("{}", status.color),
                },
                crate::status_row::StatusRow {
                    property: "Connected".into(),
                    value: format!("{}", status.connected),
                },
                crate::status_row::StatusRow {
                    property: "RSSI".into(),
                    value: status.rssi.map(|r| format!("{r} dBm")).unwrap_or_else(|| "N/A".into()),
                },
            ];
            let table = tabled::Table::new(rows).with(tabled::settings::Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "battery_level": status.battery_level.get(),
                "color": format!("{}", status.color),
                "connected": status.connected,
                "rssi": status.rssi,
            });
            println!("{json}");
        }
        OutputFormat::Plain => {
            println!("{status}");
        }
    }
}

/// Print dice color in the selected format.
pub fn print_color(color: DiceColor, format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            let row = crate::status_row::StatusRow {
                property: "Color".into(),
                value: format!("{color}"),
            };
            let table = tabled::Table::new(vec![row]).with(tabled::settings::Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            println!(r#"{{"color":"{color}"}}"#);
        }
        OutputFormat::Plain => {
            println!("{color}");
        }
    }
}

/// Parse a color string: named ("red", "green", ...) or hex ("FF0000", "0xFF0000").
pub fn parse_color(input: &str) -> Result<LedColor> {
    LedColor::from_str(input).map_err(|e| CliError::InvalidColor(e.to_string()))
}

/// Parse a dice type string ("d6", "d20", "d10", "d10x", "d4", "d8", "d12").
pub fn parse_dice_type(input: &str) -> Result<DiceType> {
    DiceType::from_str(input).map_err(|e| CliError::InvalidDiceType(e.to_string()))
}

/// Simple timestamp without external chrono dependency.
fn chrono_like_timestamp() -> String {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let h = ((secs / 3600) % 24) as u8;
    let m = ((secs / 60) % 60) as u8;
    let s = (secs % 60) as u8;
    format!("{h:02}:{m:02}:{s:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color_named() {
        assert_eq!(parse_color("red").unwrap(), LedColor::RED);
        assert_eq!(parse_color("OFF").unwrap(), LedColor::OFF);
    }

    #[test]
    fn parse_color_hex() {
        assert_eq!(parse_color("FF0000").unwrap(), LedColor::RED);
        assert_eq!(parse_color("0x00FF00").unwrap(), LedColor::GREEN);
    }

    #[test]
    fn parse_color_invalid() {
        assert!(parse_color("xyz").is_err());
    }

    #[test]
    fn parse_dice_type_valid() {
        assert_eq!(parse_dice_type("d6").unwrap(), DiceType::D6);
        assert_eq!(parse_dice_type("D20").unwrap(), DiceType::D20);
        assert_eq!(parse_dice_type("d10x").unwrap(), DiceType::D10X);
    }

    #[test]
    fn parse_dice_type_invalid() {
        assert!(parse_dice_type("d100").is_err());
    }
}
