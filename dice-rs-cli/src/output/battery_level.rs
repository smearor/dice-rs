use dice_rs::model::battery_level::BatteryLevel;
use tabled::Tabled;

use crate::cli_error::Result;
use crate::output::OutputFormatter;

/// A row in the battery level table.
#[derive(Tabled)]
pub struct BatteryRow {
    pub battery: String,
}

impl OutputFormatter for BatteryLevel {
    type Row = BatteryRow;

    fn get_table_rows(&self) -> Result<Vec<BatteryRow>> {
        Ok(vec![BatteryRow { battery: format!("{self}") }])
    }
}
