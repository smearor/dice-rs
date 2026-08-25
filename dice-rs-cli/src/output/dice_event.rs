use dice_rs::service::dice::DiceEvent;

use crate::cli_error::Result;
use crate::output::OutputFormatter;
use super::status_row::StatusRow;
use crate::timestamp::chrono_like_timestamp;

impl OutputFormatter for DiceEvent {
    type Row = StatusRow;

    fn get_table_rows(&self) -> Result<Vec<StatusRow>> {
        Ok(vec![])
    }

    fn format_table(&self) -> Result<String> {
        let now = chrono_like_timestamp();
        Ok(format!("[{now}] {self}"))
    }

    fn format_plain(&self) -> Result<String> {
        self.format_table()
    }
}
