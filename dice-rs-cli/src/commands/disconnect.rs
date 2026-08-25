use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;

pub async fn run(manager: &DiceManager, address: &str) -> Result<()> {
    manager.disconnect_by_address(address).await?;
    println!("Disconnected from {address}");
    Ok(())
}
