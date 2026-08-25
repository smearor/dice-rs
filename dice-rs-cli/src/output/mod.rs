use serde::Serialize;
use std::fmt::Display;
use tabled::Tabled;

use crate::cli_error::CliError;
use crate::cli_error::Result;
use crate::output_format::OutputFormat;

mod battery_level;
mod charging_state;
mod dice_color;
mod dice_event;
mod scan_results;
mod status_row;
mod system_status;

pub use scan_results::ScanResults;

/// Trait for formatting values in the selected output format.
///
/// Implementations provide format-specific output (table, JSON, or plain text)
/// for CLI display.
///
/// JSON output is derived automatically via `serde_json::to_string`.
/// Plain text output defaults to `Display` formatting.
/// Table output defaults to rendering `get_table_rows` via `tabled`.
/// Implementors only need to provide `get_table_rows` and set `Row`.
/// Types with non-standard table formatting (e.g. `DiceEvent`) override `format_table`.
pub trait OutputFormatter: Display + Serialize {
    /// Row type used for table rendering.
    type Row: Tabled;

    /// Build the rows for table rendering.
    fn get_table_rows(&self) -> Result<Vec<Self::Row>>;

    /// Format the value as a table.
    ///
    /// Defaults to rendering `get_table_rows` via `tabled` with rounded style.
    fn format_table(&self) -> Result<String> {
        Ok(tabled::Table::new(self.get_table_rows()?).with(tabled::settings::Style::rounded()).to_string())
    }

    /// Format the value as plain text.
    ///
    /// Defaults to `Display` formatting.
    fn format_plain(&self) -> Result<String> {
        Ok(format!("{self}"))
    }

    /// Format the value for the given output format.
    fn format_output(&self, format: OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Table => self.format_table(),
            OutputFormat::Json => serde_json::to_string(self).map_err(CliError::JsonSerialize),
            OutputFormat::Plain => self.format_plain(),
        }
    }
}

/// Print a value using its `OutputFormatter` implementation.
pub fn print<T: OutputFormatter + ?Sized>(value: &T, format: OutputFormat) {
    match value.format_output(format) {
        Ok(msg) => println!("{msg}"),
        Err(e) => eprintln!("{e}"),
    }
}
