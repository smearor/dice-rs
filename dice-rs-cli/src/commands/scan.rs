use std::time::Duration;

use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;
use crate::output;
use crate::output::ScanResults;
use crate::output_format::OutputFormat;

pub async fn run(manager: &DiceManager, duration: u64, format: OutputFormat) -> Result<()> {
    let scanner = manager.scanner().with_scan_duration(Duration::from_secs(duration));
    let devices = scanner.scan().await?;
    output::print(&ScanResults::from(devices), format);
    Ok(())
}
