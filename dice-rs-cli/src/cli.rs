use clap::Parser;

use crate::command::Command;
use crate::output_format::OutputFormat;

/// Command-line tool for GoDice BLE dice.
#[derive(Parser, Debug)]
#[command(name = "dice-rs", version, about = "Control GoDice BLE dice from the command line")]
pub struct Cli {
    /// Output format: table, json, or plain.
    #[arg(short, long, global = true, default_value = "table")]
    pub format: OutputFormat,

    /// Verbosity level (-v info, -vv debug, -vvv trace).
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Command,
}
