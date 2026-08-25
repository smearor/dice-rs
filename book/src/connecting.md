# Scanning & Connecting

## DiceScanner

`DiceScanner` discovers GoDice devices by scanning for BLE peripherals with
the `GoDice_` name prefix. It wraps the `BleTransport` trait and provides
configurable scan duration.

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;

    for device in &devices {
        println!("Found: {device}");
    }

    Ok(())
}
```

## DiceManager

`DiceManager` is the entry point for all BLE operations. It manages the BLE
adapter and provides methods for scanning, connecting, and disconnecting.

### Multi-Dice Connections

You can connect to multiple dice concurrently. Each `Dice` handle is
independent and has its own event channel:

```rust,no_run
use dice_rs::DiceManager;
use dice_rs::DiceEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;

    let mut dice_list = Vec::new();
    for device in &devices {
        let dice = manager.connect(device).await?;
        dice_list.push(dice);
    }

    // Each dice has its own event receiver
    for dice in &dice_list {
        let mut receiver = dice.subscribe();
        // Spawn a task per dice to listen for events
        let name = dice.name().to_string();
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                println!("{name}: {event}");
            }
        });
    }

    Ok(())
}
```

### Connection Retry

`DiceManager::connect()` retries up to 3 times with 1-second backoff. This
handles transient connection failures, such as a GoDice that is advertising
but not yet accepting connections while charging from 0% battery.

### Reconnect

`DiceManager::reconnect()` attempts to reconnect a disconnected dice with
exponential backoff (500ms → 5s, up to 10 retries).

### Find by Address

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let dice = manager.connect_by_address("AA:BB:CC").await?;
    println!("Connected to {}", dice.name());
    Ok(())
}
```

The address parameter accepts a partial MAC address - the first device whose
address contains the given substring is selected.

## Disconnect

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    // Disconnect all connected GoDice devices
    let count = manager.disconnect_all().await?;
    println!("Disconnected {count} dice");
    Ok(())
}
```
