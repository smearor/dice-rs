use dice_rs::service::dice::DiceDevice;
use tabled::Tabled;

/// A row in the scan results table.
#[derive(Tabled)]
pub struct DeviceRow {
    pub address: String,
    pub name: String,
    pub rssi: String,
}

impl From<&DiceDevice> for DeviceRow {
    fn from(device: &DiceDevice) -> Self {
        Self {
            address: device.address.to_string(),
            name: device.name.clone(),
            rssi: device.rssi.map(|r| format!("{r} dBm")).unwrap_or_else(|| "N/A".into()),
        }
    }
}
