use dice_rs::service::dice::DiceDevice;
use tabled::Tabled;

/// A row in the scan results table.
#[derive(Tabled)]
pub struct DeviceRow {
    pub address: String,
    pub name: String,
    pub color: String,
    pub rssi: String,
}

impl From<&DiceDevice> for DeviceRow {
    fn from(device: &DiceDevice) -> Self {
        let color = device.color().map(|c| c.to_string()).unwrap_or_else(|_| "Unknown".into());
        Self {
            address: device.address.to_string(),
            name: device.name.clone(),
            color,
            rssi: device.rssi.map(|r| format!("{r} dBm")).unwrap_or_else(|| "N/A".into()),
        }
    }
}
