mod battery;
mod calibrate;
mod charging;
mod color;
mod disconnect;
mod disconnect_all;
mod interactive;
mod led;
mod listen;
mod repl_command;
mod scan;
mod status;
mod tap;

use crate::cli_error::Result;
use crate::command::Command;
use crate::output_format::OutputFormat;
use dice_rs::service::manager::DiceManager;

/// Run a subcommand.
pub async fn run_command(manager: &DiceManager, command: Command, format: OutputFormat) -> Result<()> {
    match command {
        Command::Scan { duration } => scan::run(manager, duration, format).await,
        Command::Listen { address, dice_type } => listen::run(manager, &address, &dice_type, format).await,
        Command::Battery { address } => battery::run(manager, &address, format).await,
        Command::Led { address, action } => led::run(manager, &address, action).await,
        Command::Tap { address, enable } => tap::run(manager, &address, enable).await,
        Command::DoubleTap { address, enable } => tap::run_double(manager, &address, enable).await,
        Command::Calibrate { address } => calibrate::run(manager, &address).await,
        Command::Status { address } => status::run(manager, &address, format).await,
        Command::Color { address } => color::run(manager, &address, format).await,
        Command::Charging { address } => charging::run(manager, &address, format).await,
        Command::Disconnect { address } => disconnect::run(manager, &address).await,
        Command::DisconnectAll => disconnect_all::run(manager).await,
        Command::Interactive => interactive::run(manager).await,
    }
}
