use dice_rs::service::manager::DiceManager;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

use crate::cli_error::Result;

pub async fn run(manager: &DiceManager, address: &str) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    println!("Place the dice on a flat surface and press Enter to calibrate...");
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut input = String::new();
    reader.read_line(&mut input).await?;
    dice.calibrate().await?;
    println!("Calibration complete.");
    dice.disconnect().await?;
    Ok(())
}
