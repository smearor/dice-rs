use dice_rs::model::dice::DiceColor;

use crate::cli_error::Result;
use crate::output::OutputFormatter;
use super::status_row::StatusRow;

impl OutputFormatter for DiceColor {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![StatusRow {
            property: "Color".into(),
            value: format!("{self}"),
        }])
    }
}
