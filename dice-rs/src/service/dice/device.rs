use std::convert::TryFrom;

use btleplug::api::BDAddr;
use btleplug::platform::PeripheralId;

use crate::error::DiceError;
use crate::model::dice::DiceColor;

/// A discovered GoDice device, not yet connected.
#[derive(Debug, Clone)]
pub struct DiceDevice {
    /// Unique BLE identifier.
    pub id: PeripheralId,
    /// MAC address.
    pub address: BDAddr,
    /// Advertised device name (e.g. "GoDice_001234").
    pub name: String,
    /// Received signal strength indicator (if available).
    pub rssi: Option<i16>,
}

impl DiceDevice {
    /// Extract the physical dice color from the device name.
    ///
    /// Name format: `GoDice_{HEXID}_{COLOR}_v{VERSION}` where COLOR is a
    /// single letter: K=Black, R=Red, G=Green, B=Blue, Y=Yellow, O=Orange.
    ///
    /// Returns `DiceError::InvalidColor` if the color code cannot be parsed.
    pub fn color(&self) -> Result<DiceColor, DiceError> {
        let parts: Vec<&str> = self.name.split('_').collect();
        let code = parts.get(parts.len().saturating_sub(2)).ok_or(DiceError::InvalidColor(0))?;
        let ch = code.chars().next().ok_or(DiceError::InvalidColor(0))?;
        DiceColor::try_from(ch).map_err(|_| DiceError::InvalidColor(ch as u8))
    }
}
