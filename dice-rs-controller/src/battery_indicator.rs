use gtk4::prelude::*;

/// Battery level indicator widget with color-coded thresholds.
///
/// Cloneable — all clones share the same underlying GTK widgets.
#[derive(Clone)]
pub struct BatteryIndicator {
    level_bar: gtk4::LevelBar,
    label: gtk4::Label,
}

impl BatteryIndicator {
    /// Create a new battery indicator.
    pub fn new() -> Self {
        let level_bar = gtk4::LevelBar::builder().min_value(0.0).max_value(100.0).value(0.0).build();

        let label = gtk4::Label::builder().label("N/A").build();

        Self { level_bar, label }
    }

    /// Update the battery level display.
    pub fn set_level(&self, level: u8) {
        self.level_bar.set_value(level as f64);
        self.label.set_label(&format!("{level}%"));

        let css_class = match level {
            0..=20 => "battery-critical",
            21..=50 => "battery-low",
            _ => "battery-ok",
        };

        self.level_bar.remove_css_class("battery-critical");
        self.level_bar.remove_css_class("battery-low");
        self.level_bar.remove_css_class("battery-ok");
        self.level_bar.add_css_class(css_class);
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
