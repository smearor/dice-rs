//! dice-rs — Core library for GoDice BLE dice.
//!
//! Provides domain types, BLE transport abstraction, and a high-level
//! service API for scanning, connecting, and interacting with GoDice
//! devices over Bluetooth Low Energy.

pub mod ble;
pub mod model;
pub mod service;
