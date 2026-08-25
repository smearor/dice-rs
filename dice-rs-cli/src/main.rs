//! dice-rs-cli - Command-line tool for GoDice BLE dice.

mod cli;
mod cli_error;
mod command;
mod commands;
mod led_action;
mod output;
mod output_format;
mod timestamp;

use crate::cli::Cli;
use crate::cli_error::Result;
use crate::commands::run_command;
use clap::Parser;
use dice_rs::service::manager::DiceManager;
use tracing::Level;

fn init_logging(verbose: u8) {
    let level = match verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).with_target(false).init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose);

    let manager = DiceManager::new().await?;

    let result = run_command(&manager, cli.command, cli.format).await;

    let _ = manager.shutdown().await;

    result
}
