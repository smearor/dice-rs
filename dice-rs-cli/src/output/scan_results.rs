use std::fmt::Display;

use dice_rs::service::dice::DiceDevice;
use serde::Deserialize;
use serde::Serialize;
use tabled::Tabled;

use super::OutputFormatter;
use crate::cli_error::Result;

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

/// Wrapper for scan results to provide `Display` and `OutputFormatter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScanResults(pub Vec<DiceDevice>);

impl Display for ScanResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|d| format!("{d}")).collect::<Vec<_>>().join("\n"))
    }
}

impl From<Vec<DiceDevice>> for ScanResults {
    fn from(devices: Vec<DiceDevice>) -> Self {
        Self(devices)
    }
}

impl OutputFormatter for ScanResults {
    type Row = DeviceRow;

    fn get_table_rows(&self) -> Result<Vec<DeviceRow>> {
        Ok(self.0.iter().map(DeviceRow::from).collect())
    }

    fn format_table(&self) -> Result<String> {
        if self.0.is_empty() {
            return Ok("No GoDice devices found.".to_string());
        }
        Ok(tabled::Table::new(self.get_table_rows()?).with(tabled::settings::Style::rounded()).to_string())
    }
}
