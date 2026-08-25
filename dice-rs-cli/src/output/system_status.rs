use dice_rs::model::system_status::SystemStatus;

use crate::cli_error::Result;
use crate::output::OutputFormatter;
use super::status_row::StatusRow;

impl OutputFormatter for SystemStatus {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![
            StatusRow {
                property: "Battery".into(),
                value: format!("{}", self.battery_level),
            },
            StatusRow {
                property: "Color".into(),
                value: format!("{}", self.color),
            },
            StatusRow {
                property: "Connected".into(),
                value: format!("{}", self.connected),
            },
            StatusRow {
                property: "RSSI".into(),
                value: self.rssi.map(|r| format!("{r} dBm")).unwrap_or_else(|| "N/A".into()),
            },
        ])
    }
}
