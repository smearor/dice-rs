# dice-rs

[![Crates.io](https://img.shields.io/crates/v/dice-rs)](https://crates.io/crates/dice-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/smearor/dice-rs/actions/workflows/build.yml/badge.svg)](https://github.com/smearor/dice-rs/actions/workflows/build.yml)
[![docs.rs](https://docs.rs/dice-rs/badge.svg)](https://docs.rs/dice-rs)

A Rust library and toolkit for controlling
[GoDice](https://particula-tech.com/products/godice-full-pack) - physical
Bluetooth dice that communicate over the Nordic UART Service (NUS) BLE profile.
Scan for dice, connect, receive roll events in real time, control LEDs, query
battery level, and calibrate the accelerometer.

![dice-rs-controller](book/src/assets/dice-rs-controller.png)

## Features

- **Scan** for GoDice devices by name prefix (`GoDice_`)
- **Connect** to multiple dice concurrently with retry and backoff
- **Events** via `tokio::sync::broadcast` channel - roll start, stable face,
  tilt/fake/move stable, charging, tap, double tap, disconnect
- **LED control** - set RGB colors, pulse animations, debounced writes
- **Battery & status** - query battery level, dice color, RSSI, charging state
- **Calibration** - hardware calibration (BLE) and software calibration (offset)
- **Multi-dice types** - D6, D20, D10, D10X, D4, D8, D12 with vector tables
- **Cross-platform architecture** - `BleTransport` trait enables mock testing
  and future backends; initial release targets Linux/BlueZ

## Quick Start

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

    let dice = manager.connect(&devices[0]).await?;
    let mut receiver = dice.subscribe();
    while let Ok(event) = receiver.recv().await {
        match event {
            DiceEvent::Stable { face, .. } => println!("Rolled: {face}"),
            DiceEvent::RollStart => println!("Rolling..."),
            DiceEvent::Disconnected => break,
            _ => {}
        }
    }

    Ok(())
}
```

## Workspace Layout

| Crate | Description |
|-------|-------------|
| [`dice-rs`](./dice-rs/) | Core library: model types, BLE transport, service API |
| [`dice-rs-cli`](./dice-rs-cli/) | Command-line tool for scanning, listening, and controlling dice |
| [`dice-rs-controller`](./dice-rs-controller/) | GTK 4 desktop application with 3D dice rendering |
| [`dice-rs-ws`](./dice-rs-ws/) | WebSocket server exposing dice events over a network API |

## Documentation

- **User Guide**: [mdBook](https://smearor.github.io/dice-rs/book/)
- **API Reference**: [docs.rs](https://docs.rs/dice-rs)
- **BLE Protocol**: [BLE Specification](./docs/BLE.md)
- **Changelog**: [CHANGELOG.md](./CHANGELOG.md)

## Compatibility

- **Linux** (BlueZ 5.x with DBus) - primary target
- macOS and Windows are not supported in the initial release. The
  `BleTransport` trait allows future platform backends.

### Linux Setup

Ensure `bluetoothd` is running and the user has DBus access to the Bluetooth
adapter:

```sh
# Check BlueZ is running
systemctl status bluetooth

# The user may need to be in the bluetooth group
sudo usermod -aG bluetooth $USER
```

## Contributing

Contributions are welcome. Please read the
[Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## License

Licensed under the [MIT License](./LICENSE).