use tabled::Tabled;

/// A row in the system status table.
#[derive(Tabled)]
pub struct StatusRow {
    pub property: String,
    pub value: String,
}
