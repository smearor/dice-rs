use std::cell::Cell;

use dice_rs::model::battery_level::BatteryLevel;
use dice_rs::model::charging_state::ChargingState;
use gtk4::prelude::*;
use crate::battery_level_style::BatteryLevelStyle;

/// Battery level indicator widget with color-coded thresholds.
///
/// Cloneable - all clones share the same underlying GTK widgets.
#[derive(Clone)]
pub struct BatteryIndicator {
    level_bar: gtk4::LevelBar,
    label: gtk4::Label,
    charging: Cell<ChargingState>,
    last_level: Cell<Option<BatteryLevel>>,
}

impl BatteryIndicator {
    /// Create a new battery indicator.
    pub fn new() -> Self {
        let level_bar = gtk4::LevelBar::builder().min_value(0.0).max_value(100.0).value(0.0).build();

        let label = gtk4::Label::builder().label("N/A").build();

        Self {
            level_bar,
            label,
            charging: Cell::new(ChargingState::default()),
            last_level: Cell::new(None),
        }
    }

    /// Update the battery level display.
    pub fn set_level(&self, level: BatteryLevel) {
        self.last_level.set(Some(level));
        self.level_bar.set_value(f64::from(level.get()));

        let text = if matches!(self.charging.get(), ChargingState::Charging) {
            format!("⚡ {level}")
        } else {
            format!("{level}")
        };
        self.label.set_label(&text);

        for class in BatteryLevelStyle::all_css_classes() {
            self.level_bar.remove_css_class(class);
        }
        self.level_bar.add_css_class(BatteryLevelStyle::from(level).css_class());
    }

    /// Update the charging state display.
    /// When charging, a ⚡ prefix is shown and the `battery-charging` CSS class is applied.
    pub fn set_charging(&self, state: ChargingState) {
        self.charging.set(state);

        match state {
            ChargingState::NotCharging => {
                self.level_bar.remove_css_class("battery-charging");
            }
            ChargingState::Charging => {
                self.level_bar.add_css_class("battery-charging");
            }
        }

        if let Some(level) = self.last_level.get() {
            self.set_level(level);
        } else if state == ChargingState::Charging {
            self.label.set_label("⚡ N/A");
        }
    }

    /// Returns the label widget.
    pub fn label(&self) -> &gtk4::Label {
        &self.label
    }

    /// Returns the level bar widget.
    pub fn level_bar(&self) -> &gtk4::LevelBar {
        &self.level_bar
    }
}

impl Default for BatteryIndicator {
    fn default() -> Self {
        Self::new()
    }
}
