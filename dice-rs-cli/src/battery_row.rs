use tabled::Tabled;

/// A row in the battery level table.
#[derive(Tabled)]
pub struct BatteryRow {
    pub battery: String,
}
