use dice_rs::model::charging_state::ChargingState;

use crate::cli_error::Result;
use crate::output::OutputFormatter;
use super::status_row::StatusRow;

impl OutputFormatter for ChargingState {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![StatusRow {
            property: "Charging".into(),
            value: format!("{self}"),
        }])
    }
}
