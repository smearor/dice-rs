use dice_rs::model::battery_level::BatteryLevel;

pub struct BatteryLevelStyle(BatteryLevel);

impl BatteryLevelStyle {
    pub fn css_class(&self) -> &'static str {
        match self.0.get() {
            0..=14 => "battery-critical",
            15..=29 => "battery-low",
            _ => "battery-ok",
        }
    }

    pub fn all_css_classes() -> [&'static str; 3] {
        ["battery-critical", "battery-low", "battery-ok"]
    }
}

impl From<BatteryLevel> for BatteryLevelStyle {
    fn from(value: BatteryLevel) -> Self {
        Self(value)
    }
}
