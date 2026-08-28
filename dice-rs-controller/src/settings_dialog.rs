use std::rc::Rc;

use gtk4::prelude::*;

use crate::app_settings::AppSettings;
use crate::app_settings::AppSettingsData;

/// Settings dialog with toggle switches for UI display options.
pub struct SettingsDialog {
    dialog: gtk4::Window,
}

impl SettingsDialog {
    /// Create a new settings dialog.
    pub fn new(parent: &gtk4::ApplicationWindow, settings: &AppSettings) -> Self {
        let dialog = gtk4::Window::builder()
            .title("Settings")
            .modal(true)
            .transient_for(parent)
            .default_width(400)
            .default_height(400)
            .build();

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .margin_start(24)
            .margin_end(24)
            .margin_top(24)
            .margin_bottom(24)
            .build();

        let current = settings.get();

        let show_dice_3d_switch = gtk4::Switch::builder().active(current.show_dice_3d).build();
        content.append(&Self::create_setting_row("Show 3D dice", &show_dice_3d_switch));

        let rotate_dice_3d_switch = gtk4::Switch::builder().active(current.rotate_dice_3d).build();
        content.append(&Self::create_setting_row("Rotate 3D dice", &rotate_dice_3d_switch));

        let show_stability_switch = gtk4::Switch::builder().active(current.show_stability_indicator).build();
        content.append(&Self::create_setting_row("Show stability indicator", &show_stability_switch));

        let show_tap_switch = gtk4::Switch::builder().active(current.show_tap_controls).build();
        content.append(&Self::create_setting_row("Show tap indicator and controls", &show_tap_switch));

        let show_led_switch = gtk4::Switch::builder().active(current.show_led_controls).build();
        content.append(&Self::create_setting_row("Show LED controls", &show_led_switch));

        let show_battery_switch = gtk4::Switch::builder().active(current.show_battery_indicator).build();
        content.append(&Self::create_setting_row("Show battery indicator", &show_battery_switch));

        let show_dice_type_switch = gtk4::Switch::builder().active(current.show_dice_type_selector).build();
        content.append(&Self::create_setting_row("Show dice type selector", &show_dice_type_switch));

        let show_history_switch = gtk4::Switch::builder().active(current.show_roll_history).build();
        content.append(&Self::create_setting_row("Show roll history", &show_history_switch));

        let settings_clone = settings.clone();
        let s1 = show_dice_3d_switch.clone();
        let s2 = rotate_dice_3d_switch.clone();
        let s3 = show_stability_switch.clone();
        let s4 = show_tap_switch.clone();
        let s5 = show_led_switch.clone();
        let s6 = show_battery_switch.clone();
        let s7 = show_dice_type_switch.clone();
        let s8 = show_history_switch.clone();

        let emit_settings = Rc::new(move || {
            let data = AppSettingsData {
                show_dice_3d: s1.is_active(),
                rotate_dice_3d: s2.is_active(),
                show_stability_indicator: s3.is_active(),
                show_tap_controls: s4.is_active(),
                show_led_controls: s5.is_active(),
                show_battery_indicator: s6.is_active(),
                show_dice_type_selector: s7.is_active(),
                show_roll_history: s8.is_active(),
                compact_mode: false,
            };
            settings_clone.set(data);
        });

        let e = emit_settings.clone();
        show_dice_3d_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        rotate_dice_3d_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_stability_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_tap_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_led_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_battery_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_dice_type_switch.connect_notify_local(Some("active"), move |_, _| { e(); });
        let e = emit_settings.clone();
        show_history_switch.connect_notify_local(Some("active"), move |_, _| { e(); });

        dialog.set_child(Some(&content));

        Self { dialog }
    }

    /// Create a single setting row with a label and switch.
    fn create_setting_row(label: &str, switch: &gtk4::Switch) -> gtk4::Box {
        let row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        row.append(&gtk4::Label::builder().label(label).hexpand(true).halign(gtk4::Align::Start).build());
        switch.set_halign(gtk4::Align::End);
        row.append(switch);
        row
    }

    /// Present the dialog.
    pub fn present(&self) {
        self.dialog.present();
    }
}
