use crate::ble::ble_error::BleError;
use crate::ble::command_error::CommandError;
use std::time::Duration;

/// Errors that can occur when interacting with a GoDice.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DiceError {
    /// BLE transport error (scan, connect, disconnect, GATT operations).
    #[error(transparent)]
    Ble(#[from] BleError),
    /// A mutex lock was poisoned by a panicking thread.
    #[error("lock poisoned")]
    LockPoisoned,
    /// A request-response query timed out before the dice responded.
    #[error("response timeout: no reply within {0:?}")]
    ResponseTimeout(Duration),
    /// An invalid face value (0) was encountered.
    #[error("invalid face value: {0}")]
    InvalidFaceValue(u8),
    /// An invalid dice type byte was encountered (e.g. from AtomicU8).
    #[error("invalid dice type byte: {0}")]
    InvalidDiceType(u8),
    /// Calibration command failed. The dice reported a calibration error.
    #[error("calibration failed")]
    CalibrationFailed,
    /// The calibration protocol is not yet implemented.
    #[error("calibration protocol not yet confirmed")]
    CalibrationNotConfirmed,
    /// A notification packet could not be parsed.
    #[error("parse error: {0}")]
    Parse(#[from] crate::ble::parse_error::ParseError),
    /// A command could not be encoded or decoded.
    #[error("command error: {0}")]
    Command(#[from] CommandError),
    /// An invalid dice color byte was encountered.
    #[error("invalid dice color: {0}")]
    InvalidColor(u8),
}

/// Convenience type alias used throughout the crate.
pub type Result<T> = std::result::Result<T, DiceError>;
