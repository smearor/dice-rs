use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;

pub async fn run(manager: &DiceManager) -> Result<()> {
    let count = manager.disconnect_all().await?;
    if count == 0 {
        println!("No connected GoDice devices found.");
    } else {
        println!("Disconnected {count} GoDice device(s).");
    }
    Ok(())
}
