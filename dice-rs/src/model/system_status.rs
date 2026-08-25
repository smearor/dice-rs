use crate::model::battery_level::BatteryLevel;
use crate::model::dice::DiceColor;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Aggregated system status of a connected GoDice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct SystemStatus {
    /// Battery level (0–100 percent).
    pub battery_level: BatteryLevel,
    /// Physical dice color.
    pub color: DiceColor,
    /// Current connection state.
    pub connected: bool,
    /// Received signal strength indicator (if available).
    #[builder(default)]
    pub rssi: Option<i16>,
}

impl std::fmt::Display for SystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rssi = self.rssi.map(|r| format!("{r} dBm")).unwrap_or_else(|| "N/A".into());
        write!(f, "Battery: {}\nColor: {}\nConnected: {}\nRSSI: {}", self.battery_level, self.color, self.connected, rssi,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let status = SystemStatus::builder()
            .battery_level(BatteryLevel::new(75))
            .color(DiceColor::Green)
            .connected(true)
            .rssi(Some(-42))
            .build();
        let text = status.to_string();
        assert!(text.contains("Battery: 75%"));
        assert!(text.contains("Color: Green"));
        assert!(text.contains("Connected: true"));
        assert!(text.contains("RSSI: -42 dBm"));
    }

    #[test]
    fn display_no_rssi() {
        let status = SystemStatus::builder()
            .battery_level(BatteryLevel::new(0))
            .color(DiceColor::Red)
            .connected(false)
            .build();
        let text = status.to_string();
        assert!(text.contains("RSSI: N/A"));
    }
}
