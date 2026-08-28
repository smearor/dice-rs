use std::cell::RefCell;
use std::rc::Rc;

use crate::config::config_dir::load_app_settings;
use crate::config::config_dir::save_app_settings;
use serde::Deserialize;
use serde::Serialize;

/// Application display settings data.
///
/// Controls visibility of various UI elements in the dice controller.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettingsData {
    /// Whether the 3D dice view is shown.
    pub show_dice_3d: bool,
    /// Whether the 3D dice rotates continuously.
    pub rotate_dice_3d: bool,
    /// Whether the stability indicator is shown.
    pub show_stability_indicator: bool,
    /// Whether the tap indicator and tap controls are shown.
    pub show_tap_controls: bool,
    /// Whether the LED controls are shown.
    pub show_led_controls: bool,
    /// Whether the battery indicator is shown.
    pub show_battery_indicator: bool,
    /// Whether the dice type selector is shown.
    pub show_dice_type_selector: bool,
    /// Whether the roll history is shown.
    pub show_roll_history: bool,
    /// Whether compact mode is enabled (single-line rows).
    pub compact_mode: bool,
}

impl Default for AppSettingsData {
    fn default() -> Self {
        Self {
            show_dice_3d: true,
            rotate_dice_3d: true,
            show_stability_indicator: true,
            show_tap_controls: false,
            show_led_controls: true,
            show_battery_indicator: true,
            show_dice_type_selector: true,
            show_roll_history: true,
            compact_mode: false,
        }
    }
}

/// Type alias for the settings change listener callback.
type SettingsListener = Box<dyn Fn(AppSettingsData) + 'static>;

/// Shared, cloneable application settings with change notification.
///
/// All clones share the same underlying state. When settings are updated
/// via [`AppSettings::set`], all registered listeners are invoked.
#[derive(Clone)]
pub struct AppSettings {
    data: Rc<RefCell<AppSettingsData>>,
    listeners: Rc<RefCell<Vec<SettingsListener>>>,
}

impl AppSettings {
    /// Create a new settings instance, loading from disk if available.
    pub fn new() -> Self {
        let data = load_app_settings().unwrap_or_default();
        Self {
            data: Rc::new(RefCell::new(data)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get a snapshot of the current settings.
    pub fn get(&self) -> AppSettingsData {
        self.data.borrow().clone()
    }

    /// Update settings and notify all listeners.
    pub fn set(&self, data: AppSettingsData) {
        *self.data.borrow_mut() = data.clone();
        save_app_settings(&data);
        for listener in self.listeners.borrow().iter() {
            listener(data.clone());
        }
    }

    /// Register a callback that is invoked whenever settings change.
    pub fn connect_changed<F>(&self, callback: F)
    where
        F: Fn(AppSettingsData) + 'static,
    {
        self.listeners.borrow_mut().push(Box::new(callback));
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new()
    }
}
