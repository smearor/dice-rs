use std::str::FromStr;

/// Commands available in the interactive REPL.
#[derive(Debug, PartialEq, Eq)]
pub enum ReplCommand {
    /// Show help text.
    Help,
    /// Exit the REPL.
    Quit,
    /// Scan for GoDice devices.
    Scan,
    /// Connect to a device by MAC address.
    Connect(String),
    /// Disconnect from the current device.
    Disconnect,
    /// Query battery level.
    Battery,
    /// Query dice color.
    Color,
    /// Check charging state.
    Charging,
    /// Set LEDs to a color.
    Led(String),
    /// Show system status.
    Status,
    /// Calibrate the dice.
    Calibrate,
}

/// Error returned when parsing an invalid REPL command.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown command: {0} (type 'help' for available commands)")]
pub struct ReplCommandError(String);

impl FromStr for ReplCommand {
    type Err = ReplCommandError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        match input {
            "help" => Ok(Self::Help),
            "quit" | "exit" => Ok(Self::Quit),
            "scan" => Ok(Self::Scan),
            "disconnect" => Ok(Self::Disconnect),
            "battery" => Ok(Self::Battery),
            "color" => Ok(Self::Color),
            "charging" => Ok(Self::Charging),
            "status" => Ok(Self::Status),
            "calibrate" => Ok(Self::Calibrate),
            _ if input.starts_with("connect ") => {
                let address = input.strip_prefix("connect ").unwrap_or(input).trim();
                Ok(Self::Connect(address.to_string()))
            }
            _ if input.starts_with("led ") => {
                let color = input.strip_prefix("led ").unwrap_or(input).trim();
                Ok(Self::Led(color.to_string()))
            }
            _ => Err(ReplCommandError(input.to_string())),
        }
    }
}

impl ReplCommand {
    /// Print the help text for available REPL commands.
    pub fn print_help() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_help() {
        assert_eq!(ReplCommand::from_str("help").unwrap(), ReplCommand::Help);
    }

    #[test]
    fn parse_quit() {
        assert_eq!(ReplCommand::from_str("quit").unwrap(), ReplCommand::Quit);
        assert_eq!(ReplCommand::from_str("exit").unwrap(), ReplCommand::Quit);
    }

    #[test]
    fn parse_scan() {
        assert_eq!(ReplCommand::from_str("scan").unwrap(), ReplCommand::Scan);
    }

    #[test]
    fn parse_connect() {
        let cmd = ReplCommand::from_str("connect AA:BB:CC").unwrap();
        assert_eq!(cmd, ReplCommand::Connect("AA:BB:CC".to_string()));
    }

    #[test]
    fn parse_led() {
        let cmd = ReplCommand::from_str("led red").unwrap();
        assert_eq!(cmd, ReplCommand::Led("red".to_string()));
    }

    #[test]
    fn parse_unknown() {
        assert!(ReplCommand::from_str("foobar").is_err());
    }
}
