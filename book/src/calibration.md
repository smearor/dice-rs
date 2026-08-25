# Calibration

## Software Calibration

Software calibration computes an `AccelerationOffset` from the next `Stable`
event. The offset is the difference between the measured acceleration and
the expected ideal gravity vector for the current dice type. The offset is
then subtracted from all subsequent accelerometer readings before face value
interpretation.

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    println!("Place the dice on a flat surface, then press Enter...");
    // In a real app, wait for user input

    let offset = dice.calibrate_software().await?;
    println!("Calibration offset: {offset:?}");

    Ok(())
}
```

After calibration, all subsequent `Stable`, `TiltStable`, `FakeStable`, and
`MoveStable` events use the corrected acceleration data for face value
determination.

## Clear Calibration

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    dice.calibrate_software().await?;
    // ... later ...
    dice.clear_software_calibration()?;

    Ok(())
}
```

## Hardware Calibration

Hardware calibration sends a BLE command (opcode `0x13`) to the dice. The
exact byte encoding is unconfirmed - this method is tentative and may not
work with all firmware versions.

```rust,no_run
use dice_rs::DiceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = DiceManager::new().await?;
    let devices = manager.scan().await?;
    let dice = manager.connect(&devices[0]).await?;

    dice.calibrate().await?;
    println!("Hardware calibration complete");

    Ok(())
}
```

If the dice reports a calibration failure, `DiceError::CalibrationFailed` is
returned. If the dice does not respond within 5 seconds,
`DiceError::ResponseTimeout` is returned.
