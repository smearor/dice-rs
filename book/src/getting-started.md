# Getting Started

## Add the Dependency

Add `dice-rs` to your `Cargo.toml`:

```toml
[dependencies]
dice-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Tokio Runtime

`dice-rs` is async-only. You need a tokio runtime to drive the BLE operations:

```rust
use dice_rs::{DiceManager, DiceEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;

    if devices.is_empty() {
        println!("No GoDice devices found");
        return Ok(());
    }

    for device in &devices {
        println!("Found: {device}");
    }

    let dice = manager.connect(&devices[0]).await?;
    let mut receiver = dice.subscribe();
    while let Ok(event) = receiver.recv().await {
        match event {
            DiceEvent::Stable { face, .. } => {
                println!("Rolled: {face}");
                break;
            }
            DiceEvent::RollStart => println!("Rolling..."),
            DiceEvent::Disconnected => break,
            _ => {}
        }
    }

    dice.disconnect().await?;
    Ok(())
}
```

## First Connection

1. Create a `DiceManager` - this initializes the BLE adapter.
2. Call `scan()` to discover nearby GoDice devices (filtered by `GoDice_` prefix).
3. Call `connect()` with a `DiceDevice` to establish a BLE connection.
4. Call `subscribe()` to get a `broadcast::Receiver<DiceEvent>`.
5. Listen for events in a loop.

The `Dice` handle is `Clone`, so you can share it across tasks. All clones
share the same underlying connection state.
