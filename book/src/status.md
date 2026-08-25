# Battery & Status

## Battery Level

Query the battery level (0–100 percent):

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let battery = dice.get_battery_level().await?;
    println!("Battery: {battery}");

    Ok(())
}
```

The request uses a `oneshot` channel with a 5-second timeout. If the dice
does not respond, `DiceError::ResponseTimeout` is returned.

## Dice Color

Query the physical shell color:

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let color = dice.get_color().await?;
    println!("Color: {color}");

    Ok(())
}
```

## Charging State

The charging state is updated automatically from notifications. Query the
last known state without sending a BLE command:

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let charging = dice.charging_state();
    println!("Charging: {charging}");

    Ok(())
}
```

Charging state changes are also delivered as `DiceEvent::Charging` events.

## RSSI

Query the signal strength (if available):

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    if let Some(rssi) = dice.rssi().await? {
        println!("RSSI: {rssi} dBm");
    }

    Ok(())
}
```

RSSI availability depends on the Bluetooth adapter and BlueZ version.

## System Status

Get all status information in a single call:

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    let status = dice.system_status().await?;
    println!("Battery: {}", status.battery_level);
    println!("Color: {}", status.color);
    println!("Connected: {}", status.connected);
    if let Some(rssi) = status.rssi {
        println!("RSSI: {rssi} dBm");
    }

    Ok(())
}
```

`system_status()` performs battery level and color queries concurrently for
efficiency.
