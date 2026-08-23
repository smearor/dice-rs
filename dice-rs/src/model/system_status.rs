use crate::model::color::DieColor;

/// Aggregated system status of a connected GoDice.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemStatus {
    /// Battery level (0–100 percent).
    pub battery_level: u8,
    /// Physical dice color.
    pub color: DieColor,
    /// Current connection state.
    pub connected: bool,
    /// Received signal strength indicator (if available).
    pub rssi: Option<i16>,
}
