# Dice Events

## Event Types

`DiceEvent` is the high-level event enum emitted by a connected GoDice.
Events are delivered via a `tokio::sync::broadcast` channel.

| Variant | Description |
|---------|-------------|
| `RollStart` | Dice has started rolling |
| `Stable { face, acceleration }` | Dice is stable and flat after a roll |
| `TiltStable { face, acceleration }` | Stable but tilted |
| `FakeStable { face, acceleration }` | Stable after a fake roll |
| `MoveStable { face, acceleration }` | Stable after small movement |
| `Charging { state }` | Charging status changed |
| `Tap` | Single tap detected (must be enabled) |
| `DoubleTap` | Double tap detected (must be enabled) |
| `Disconnected` | BLE link lost or dice disconnected |

## Subscribing to Events

```rust,no_run
use dice_rs::{DiceManager, DiceEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let mut receiver = dice.subscribe();
    while let Ok(event) = receiver.recv().await {
        match event {
            DiceEvent::Stable { face, acceleration } => {
                println!("Stable on face {face} (accel: {acceleration:?})");
            }
            DiceEvent::RollStart => println!("Rolling..."),
            DiceEvent::TiltStable { face, .. } => println!("Tilt stable: {face}"),
            DiceEvent::FakeStable { face, .. } => println!("Fake stable: {face}"),
            DiceEvent::MoveStable { face, .. } => println!("Move stable: {face}"),
            DiceEvent::Charging { state } => println!("Charging: {state}"),
            DiceEvent::Tap => println!("Tap!"),
            DiceEvent::DoubleTap => println!("Double tap!"),
            DiceEvent::Disconnected => {
                println!("Disconnected");
                break;
            }
        }
    }

    Ok(())
}
```

## Multiple Subscribers

The broadcast channel supports multiple subscribers. Each call to
`subscribe()` returns an independent receiver:

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let mut rx1 = dice.subscribe();
    let mut rx2 = dice.subscribe();

    // Both receivers get the same events
    tokio::spawn(async move {
        while let Ok(event) = rx1.recv().await {
            println!("Subscriber 1: {event}");
        }
    });

    while let Ok(event) = rx2.recv().await {
        println!("Subscriber 2: {event}");
    }

    Ok(())
}
```

## Accelerometer Data

`Stable`, `TiltStable`, `FakeStable`, and `MoveStable` events include an
`Acceleration` struct with raw XYZ accelerometer data (three `i8` values).
The face value is computed by finding the closest reference vector in the
dice type's vector table. See [BLE Protocol](./ble-protocol.md) for the
full vector tables.

## Tap Events

Tap and double tap notifications are disabled by default. Enable them
explicitly:

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    dice.enable_tap().await?;
    dice.enable_double_tap().await?;

    let mut receiver = dice.subscribe();
    while let Ok(event) = receiver.recv().await {
        println!("{event}");
    }

    Ok(())
}
```
