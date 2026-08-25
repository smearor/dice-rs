# Introduction

`dice-rs` is a Rust library and toolkit for controlling
[GoDice](https://particula-tech.com/products/godice-full-pack) - physical
Bluetooth dice that communicate over Bluetooth Low Energy (BLE).

## Motivation

GoDice are physical dice with embedded LEDs, an accelerometer, and a BLE
radio. They use the Nordic UART Service (NUS) profile for communication.
The official APIs from Particula are in JavaScript and Python. `dice-rs`
brings native Rust support with an idiomatic async API, type-safe domain
model, and a trait-based transport layer for testability.

## Scope

The workspace provides four crates:

- **`dice-rs`** - core library with domain types, BLE transport, and a
  high-level service API
- **`dice-rs-cli`** - command-line tool for quick interactions
- **`dice-rs-controller`** - GTK 4 desktop application with 3D dice rendering
- **`dice-rs-ws`** - WebSocket server for network-accessible dice events

## Where to Get Help

- [GitHub Issues](https://github.com/smearor/dice-rs/issues) - bug reports
  and feature requests
- [BLE Protocol](./ble-protocol.md) - canonical protocol reference
- [API Reference (docs.rs)](https://docs.rs/dice-rs) - rustdoc for all crates
