use btleplug::api::BDAddr;
use btleplug::platform::PeripheralId;

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
