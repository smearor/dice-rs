use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;
use crate::output;
use crate::output_format::OutputFormat;

pub async fn run(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    let level = dice.get_battery_level().await?;
    output::print(&level, format);
    dice.disconnect().await?;
    Ok(())
}
