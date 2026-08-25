use std::str::FromStr;

use dice_rs::model::dice::DiceType;
use dice_rs::service::dice::DiceEvent;
use dice_rs::service::manager::DiceManager;
use tokio::sync::broadcast;
use tracing::debug;

use crate::cli_error::Result;
use crate::output;
use crate::output_format::OutputFormat;

pub async fn run(manager: &DiceManager, address: &str, dice_type: &str, format: OutputFormat) -> Result<()> {
    let dice = manager.connect_by_address(address).await?;
    let dt = DiceType::from_str(dice_type)?;
    dice.set_dice_type(dt);

    let mut events = dice.subscribe();
    println!("Listening for events from {address} (Ctrl+C to stop)...");

    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    Ok(event) => {
                        output::print(&event, format);
                        if matches!(event, DiceEvent::Disconnected) {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        debug!("missed {n} events");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nStopping...");
                break;
            }
        }
    }

    dice.disconnect().await?;
    Ok(())
}
