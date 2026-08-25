use dice_rs::service::manager::DiceManager;

use crate::cli_error::Result;

pub async fn run(manager: &DiceManager, address: &str, enable: bool) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    if enable {
        dice.enable_tap().await?;
        println!("Tap notifications enabled for {address}");
    } else {
        dice.disable_tap().await?;
        println!("Tap notifications disabled for {address}");
    }
    dice.disconnect().await?;
    Ok(())
}

pub async fn run_double(manager: &DiceManager, address: &str, enable: bool) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    if enable {
        dice.enable_double_tap().await?;
        println!("Double tap notifications enabled for {address}");
    } else {
        dice.disable_double_tap().await?;
        println!("Double tap notifications disabled for {address}");
    }
    dice.disconnect().await?;
    Ok(())
}
