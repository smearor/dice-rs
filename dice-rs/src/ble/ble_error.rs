use crate::ble::nus_characteristic::NusCharacteristic;

/// Errors originating from the BLE transport layer.
///
/// Each variant carries the underlying backend error message as a `String`
/// for diagnostics, without exposing the backend (btleplug) error type to
/// consumers of this crate.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum BleError {
    /// BLE scan or adapter operation failed (start, stop, enumerate, events).
    #[error("BLE scan failed: {0}")]
    Scan(String),

    /// BLE connection attempt failed (backend error).
    #[error("connection failed: {0}")]
    Connect(String),

    /// No connection attempt was made yet.
    #[error("no connection attempt made")]
    NoAttemptMade,

    /// Device with the given address was not found in scan results.
    #[error("device not found: {address}")]
    DeviceNotFound { address: String },

    /// Peripheral was not found among discovered BLE peripherals.
    #[error("peripheral not found for {name}")]
    PeripheralNotFound { name: String },

    /// BLE disconnect operation failed.
    #[error("disconnect failed: {0}")]
    Disconnect(String),

    /// BLE connection state check or property query failed while not connected.
    #[error("not connected: {0}")]
    NotConnected(String),

    /// GATT service discovery failed.
    #[error("service discovery failed: {0}")]
    Discovery(String),

    /// A required GATT characteristic was not found.
    #[error("characteristic not found: {0}")]
    CharacteristicNotFound(NusCharacteristic),

    /// GATT characteristic write failed.
    #[error("write failed: {0}")]
    Write(String),

    /// GATT subscribe or notify operation failed.
    #[error("subscribe failed: {0}")]
    Subscribe(String),

    /// Connection was lost during an operation.
    #[error("connection lost")]
    ConnectionLost,

    /// Reconnect attempts exhausted without success.
    #[error("reconnect failed after max retries")]
    ReconnectFailed,
}

impl BleError {
    /// Create a `Scan` error from any displayable error.
    pub fn scan(e: impl std::fmt::Display) -> Self {
        Self::Scan(e.to_string())
    }

    /// Create a `Connect` error from any displayable error.
    pub fn connect(e: impl std::fmt::Display) -> Self {
        Self::Connect(e.to_string())
    }

    /// Create a `DeviceNotFound` error for the given address.
    pub fn device_not_found(address: impl std::fmt::Display) -> Self {
        Self::DeviceNotFound { address: address.to_string() }
    }

    /// Create a `PeripheralNotFound` error for the given device name.
    pub fn peripheral_not_found(name: impl std::fmt::Display) -> Self {
        Self::PeripheralNotFound { name: name.to_string() }
    }

    /// Create a `Disconnect` error from any displayable error.
    pub fn disconnect(e: impl std::fmt::Display) -> Self {
        Self::Disconnect(e.to_string())
    }

    /// Create a `NotConnected` error from any displayable error.
    pub fn not_connected(e: impl std::fmt::Display) -> Self {
        Self::NotConnected(e.to_string())
    }

    /// Create a `Discovery` error from any displayable error.
    pub fn discovery(e: impl std::fmt::Display) -> Self {
        Self::Discovery(e.to_string())
    }

    /// Create a `Write` error from any displayable error.
    pub fn write(e: impl std::fmt::Display) -> Self {
        Self::Write(e.to_string())
    }

    /// Create a `Subscribe` error from any displayable error.
    pub fn subscribe(e: impl std::fmt::Display) -> Self {
        Self::Subscribe(e.to_string())
    }

    /// Create a `CharacteristicNotFound` error for the given NUS characteristic.
    pub fn characteristic_not_found(char: NusCharacteristic) -> Self {
        Self::CharacteristicNotFound(char)
    }
}
