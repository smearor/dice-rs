use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::MaybeUninit;

use btleplug::api::BDAddr;
use btleplug::platform::PeripheralId;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::error::DiceError;
use crate::model::dice::DiceColor;

/// A discovered GoDice device, not yet connected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceDevice {
    /// Unique BLE identifier (not serialized).
    #[serde(skip, default = "default_peripheral_id")]
    pub id: PeripheralId,
    /// MAC address.
    #[serde(serialize_with = "serialize_bdaddr", deserialize_with = "deserialize_bdaddr")]
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

impl Display for DiceDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = self.color().map(|c| c.to_string()).unwrap_or_else(|_| "Unknown".into());
        let rssi = self.rssi.map(|r| format!("{r}")).unwrap_or_else(|| "N/A".into());
        write!(f, "{} {} {} {}", self.address, self.name, color, rssi)
    }
}

/// Produce a sentinel `PeripheralId` for deserialization contexts where the real
/// BLE identifier is not available (e.g. JSON from a client).
///
/// `PeripheralId` wraps a `DeviceId` with no public constructor, so we use
/// `MaybeUninit::zeroed` to create an inert placeholder. The `id` field is
/// only meaningful for live BLE operations and is never sent to clients.
fn default_peripheral_id() -> PeripheralId {
    // SAFETY: `PeripheralId` is a thin wrapper around `DeviceId` which contains
    // a `dbus::Path<'static>` (a `String`-like type). Zeroing produces an
    // empty path, which is a valid but meaningless value. This placeholder is
    // only used when deserializing `DiceDevice` from JSON where `id` was
    // skipped; it should never be used to address a real BLE device.
    unsafe { MaybeUninit::zeroed().assume_init() }
}

/// Serialize a `BDAddr` as a colon-separated MAC string (e.g. "AA:BB:CC:DD:EE:FF").
fn serialize_bdaddr<S>(addr: &BDAddr, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = addr.as_ref();
    let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":");
    serializer.serialize_str(&hex)
}

/// Deserialize a `BDAddr` from a MAC string (e.g. "AA:BB:CC:DD:EE:FF").
fn deserialize_bdaddr<'de, D>(deserializer: D) -> Result<BDAddr, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let bytes: Vec<u8> = s
        .split(':')
        .map(|part| u8::from_str_radix(part, 16).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    if bytes.len() != 6 {
        return Err(serde::de::Error::custom("BDAddr must be 6 bytes"));
    }
    let arr: [u8; 6] = bytes.as_slice().try_into().map_err(serde::de::Error::custom)?;
    Ok(BDAddr::from(arr))
}
