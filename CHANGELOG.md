# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

### Fixed

### Distribution

### Infrastructure


## 0.1.0 - 2026-08-28

### Added

- BLE transport abstraction with `BleTransport` and `BlePeripheral` traits
- `BtleplugTransport` implementation for Linux (BlueZ DBus)
- `DiceScanner` with name-prefix filtering for `GoDice_` devices
- `DiceManager` for multi-dice connection management with retry and backoff
- `Dice` handle with event channel (`broadcast::Receiver<DiceEvent>`)
- `DiceEvent` enum with 9 variants (RollStart, Stable, TiltStable, FakeStable, MoveStable, Charging, Tap, DoubleTap, Disconnected)
- `DiceType` enum (D6, D20, D10, D10X, D4, D8, D12) with vector tables and shell transforms
- `LedColor` struct with RGB constants, hex parsing, and named color parsing
- `PulseBlinkMode` and `PulseLeds` enums for pulse LED animations
- `Command` enum encoding all host-to-dice BLE commands
- `Event` enum decoding all dice-to-host BLE notifications
- `Acceleration` struct with face value interpretation via vector tables
- `BatteryLevel`, `ChargingState`, `DiceColor`, `FaceValue` domain types
- `SystemStatus` aggregating battery, color, connection, and RSSI
- `DiceError` with `thiserror` and 10+ variants
- LED debounce (30ms coalescing) to prevent BlueZ/DBus socket buffer overflow
- Software calibration via `AccelerationOffset` computation
- Hardware calibration via BLE command (tentative)
- Tap and double tap interrupt enable/disable
- Connection monitor with periodic health check
- Reconnect with exponential backoff (500ms → 5s, up to 10 retries)
- `dice-rs-cli` with scan, listen, battery, led, tap, double-tap, calibrate, status, color, charging, disconnect, disconnect-all, interactive subcommands
- `dice-rs-controller` GTK 4 application with 3D dice rendering, LED controls, tap controls, battery indicator, roll history
- `dice-rs-ws` WebSocket server with REST API and real-time event streaming
- mdBook user guide with 13 chapters
- Root `README.md` with badges, quick start, and workspace layout

### Infrastructure

- Cargo workspace with 4 crates
- GitHub Actions: fmt, clippy, test, audit, mdBook build, docs build
- `BleTransport` trait for testability and future platform backends
- Cross-platform CI: Windows and macOS runners for `dice-rs`, `dice-rs-cli`, and `dice-rs-ws` (build + test)
- Linux aarch64 CI: native `ubuntu-24.04-arm` runner for lint, build, and test
- CI matrix: `lint` and `test` jobs now run on `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`
- CI matrix: `build_linux` renamed from `build`, `build_cross_platform` and `test_cross_platform` jobs added
- MSRV 1.88
