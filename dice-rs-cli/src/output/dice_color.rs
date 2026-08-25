use dice_rs::model::dice::DiceColor;

use super::status_row::StatusRow;
use crate::cli_error::Result;
use crate::output::OutputFormatter;

impl OutputFormatter for DiceColor {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![StatusRow {
            property: "Color".into(),
            value: format!("{self}"),
        }])
    }
}
