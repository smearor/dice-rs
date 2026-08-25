use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;
use crate::output;
use crate::output_format::OutputFormat;

pub async fn run(manager: &DiceManager, address: &str, format: OutputFormat) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    let charging = dice.charging_state();
    output::print(&charging, format);
    dice.disconnect().await?;
    Ok(())
}
