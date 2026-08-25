use dice_rs::model::charging_state::ChargingState;

use super::status_row::StatusRow;
use crate::cli_error::Result;
use crate::output::OutputFormatter;

impl OutputFormatter for ChargingState {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![StatusRow {
            property: "Charging".into(),
            value: format!("{self}"),
        }])
    }
}
