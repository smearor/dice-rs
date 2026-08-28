use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

use dice_rs::model::stability_descriptor::StabilityDescriptor;
use dice_rs::service::dice::Dice;
use dice_rs::service::manager::DiceManager;
use gtk4::glib;
use gtk4::prelude::*;

use crate::config::app_settings::AppSettingsData;
use crate::platform::drag_reorder::setup_drag_reorder;
use crate::platform::event_controller::EventController;
use crate::platform::ui_update::UiUpdate;
use crate::platform::widget_container::WidgetContainer;
use crate::services::dice_service::DiceService;
use crate::styling::dice::DiceColorStyle;
use crate::widgets::battery_indicator::BatteryIndicator;
use crate::widgets::dice_3d::Dice3D;
use crate::widgets::dice_type_selector::DiceTypeSelector;
use crate::widgets::face_display::FaceDisplay;
use crate::widgets::led_controls::LedControls;
use crate::widgets::roll_history::RollHistory;
use crate::widgets::tap_controls::TapControls;
use crate::widgets::tap_indicator::TapIndicator;

/// A list row representing a single connected dice.
///
/// Displays all UI elements for one dice: 3D view, face value, stability,
/// battery, roll history, LED controls, tap indicator/controls, and dice
/// type selector. Supports a compact single-line mode.
pub struct DiceRow {
    /// The main horizontal container holding the 3D frame and info box.
    container: gtk4::Box,
    /// The 3D dice renderer widget.
    dice_3d: Dice3D,
    /// Frame wrapping the 3D dice view on the left side.
    dice_3d_frame: gtk4::Frame,
    /// Label displaying the current stability descriptor.
    stability_label: gtk4::Label,
    /// Widget showing transient tap and double-tap notifications.
    tap_indicator: TapIndicator,
    /// Widget with switches to enable/disable tap and double-tap interrupts.
    tap_controls: TapControls,
    /// LED control panel widget.
    led_controls: LedControls,
    /// Battery indicator widget.
    battery_indicator: BatteryIndicator,
    /// Dropdown for selecting the dice shell type (D6, D20, etc.).
    dice_type_selector: DiceTypeSelector,
    /// Roll history widget.
    roll_history: RollHistory,
    /// Compact single-line row shown when compact mode is enabled.
    compact_box: gtk4::Box,
}

impl DiceRow {
    /// Create a new dice row for a connected dice.
    pub fn new(dice: Dice, manager: Arc<DiceManager>) -> Self {
        let dice_service = DiceService::new(dice.clone());
        let face_display = FaceDisplay::new();
        let battery_indicator = BatteryIndicator::new();
        let led_controls = LedControls::new(dice_service.clone());
        let dice_3d = Dice3D::new();
        let roll_history = RollHistory::new();
        let tap_indicator = TapIndicator::new();
        let tap_controls = TapControls::new(dice_service.clone());

        // Apply persisted settings to 3D renderer and LED color pickers.
        let dice_settings = dice_service.load_settings();
        dice_3d.set_dice_type(dice_settings.dice_type);
        led_controls.set_colors(dice_settings.led_color1, dice_settings.led_color2);

        // Left side: 3D dice view.
        let dice_3d_frame = gtk4::Frame::builder()
            .css_classes(vec!["dice-3d-frame"])
            .width_request(120)
            .height_request(120)
            .child(dice_3d.widget())
            .build();

        // Right side: face value, stability, battery, history, LED controls.
        let info_box = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).hexpand(true).build();

        let dice_type_selector = DiceTypeSelector::new(&dice_service, &dice_3d);

        let face_widget = face_display.widget();
        face_widget.add_css_class("face-display-frame");
        face_widget.set_size_request(80, 80);
        face_widget.set_margin_top(8);
        face_widget.set_hexpand(false);
        face_widget.set_vexpand(false);

        let header = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
        header.append(face_widget);

        let stability_label = face_display.stability_label();
        stability_label.set_valign(gtk4::Align::Center);
        stability_label.set_xalign(0.5);
        stability_label.set_halign(gtk4::Align::Center);
        stability_label.add_css_class("stability-label-frame");
        stability_label.set_size_request(80, 80);
        stability_label.set_margin_top(8);
        stability_label.set_hexpand(false);
        stability_label.set_vexpand(false);
        header.append(stability_label);

        // Vertical box combining roll history and battery indicator.
        let history_battery_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(4)
            .valign(gtk4::Align::Center)
            .hexpand(true)
            .build();
        roll_history.pack_into(&history_battery_box);
        battery_indicator.pack_into(&history_battery_box);
        header.append(&history_battery_box);

        tap_indicator.widget().set_valign(gtk4::Align::Center);
        tap_indicator.pack_into(&header);

        dice_type_selector.widget().set_valign(gtk4::Align::Center);
        dice_type_selector.pack_into(&header);

        info_box.append(&header);

        let controls_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_bottom(8)
            .build();
        led_controls.pack_into(&controls_row);
        tap_controls.widget().set_hexpand(true);
        tap_controls.widget().set_halign(gtk4::Align::End);
        tap_controls.pack_into(&controls_row);
        info_box.append(&controls_row);

        // Compact mode: square face display only, arranged horizontally.
        let compact_face = gtk4::Label::builder().label("?").css_classes(vec!["face-display", "face-unknown"]).build();
        compact_face.set_size_request(80, 80);
        compact_face.set_hexpand(false);
        compact_face.set_vexpand(false);
        let compact_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .margin_start(0)
            .margin_end(0)
            .margin_top(6)
            .margin_bottom(6)
            .css_classes(vec!["dice-row", "compact-row"])
            .build();
        compact_box.append(&compact_face);

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(0)
            .margin_bottom(0)
            .css_classes(vec!["dice-row"])
            .build();
        container.append(&dice_3d_frame);
        container.append(&info_box);

        // Apply colored border based on dice physical color.
        let container_clone = container.clone();
        let compact_box_clone = compact_box.clone();
        let face_display_for_color = face_display.clone();
        let roll_history_for_color = roll_history.clone();
        let dice_3d_for_color = dice_3d.clone();
        let dice_service_for_color = dice_service.clone();
        glib::spawn_future_local(async move {
            if let Ok(color) = dice_service_for_color.get_color().await {
                let style = DiceColorStyle::from(color);
                container_clone.add_css_class(style.border_css_class());
                compact_box_clone.add_css_class(style.border_css_class());
                face_display_for_color.set_dice_color(color);
                roll_history_for_color.set_dice_color(color);
                dice_3d_for_color.set_color(color);
            }
        });

        let controller = EventController::new(dice, manager);
        let (sender, receiver) = mpsc::channel::<UiUpdate>();
        controller.start(sender);

        // GTK main thread: poll the channel and apply UI updates to widgets.
        let face_display_for_events = face_display.clone();
        let dice_3d_for_events = dice_3d.clone();
        let roll_history_for_events = roll_history.clone();
        let battery_indicator_for_events = battery_indicator.clone();
        let tap_indicator_for_events = tap_indicator.clone();
        let compact_label_for_events = compact_face.clone();
        glib::timeout_add_local(Duration::from_millis(10), move || {
            while let Ok(update) = receiver.try_recv() {
                match update {
                    UiUpdate::Rolling => {
                        face_display_for_events.set_rolling();
                        face_display_for_events.set_stability(StabilityDescriptor::Rolling);
                        compact_label_for_events.set_text("...");
                    }
                    UiUpdate::Stable { face, acceleration } => {
                        face_display_for_events.set_face(face);
                        face_display_for_events.set_stability(StabilityDescriptor::Stable);
                        roll_history_for_events.add_roll(face, StabilityDescriptor::Stable);
                        dice_3d_for_events.set_orientation(acceleration);
                        compact_label_for_events.set_text(&face.to_string());
                    }
                    UiUpdate::TiltStable { face, acceleration } => {
                        face_display_for_events.set_face(face);
                        face_display_for_events.set_tilted(true);
                        face_display_for_events.set_stability(StabilityDescriptor::TiltStable);
                        roll_history_for_events.add_roll(face, StabilityDescriptor::TiltStable);
                        dice_3d_for_events.set_orientation(acceleration);
                        compact_label_for_events.set_text(&face.to_string());
                    }
                    UiUpdate::FakeStable { face, acceleration } => {
                        face_display_for_events.set_face(face);
                        face_display_for_events.set_fake(true);
                        face_display_for_events.set_stability(StabilityDescriptor::FakeStable);
                        roll_history_for_events.add_roll(face, StabilityDescriptor::FakeStable);
                        dice_3d_for_events.set_orientation(acceleration);
                        compact_label_for_events.set_text(&face.to_string());
                    }
                    UiUpdate::MoveStable { face, acceleration } => {
                        face_display_for_events.set_face(face);
                        face_display_for_events.set_stability(StabilityDescriptor::MoveStable);
                        roll_history_for_events.add_roll(face, StabilityDescriptor::MoveStable);
                        dice_3d_for_events.set_orientation(acceleration);
                        compact_label_for_events.set_text(&face.to_string());
                    }
                    UiUpdate::Charging { state } => {
                        battery_indicator_for_events.set_charging(state);
                    }
                    UiUpdate::Tap => {
                        tap_indicator_for_events.flash_tap();
                    }
                    UiUpdate::DoubleTap => {
                        tap_indicator_for_events.flash_double_tap();
                    }
                    UiUpdate::Disconnected => {
                        face_display_for_events.set_disconnected();
                        face_display_for_events.set_stability(StabilityDescriptor::Rolling);
                        compact_label_for_events.set_text("-");
                    }
                    UiUpdate::BatteryLevel(level) => {
                        battery_indicator_for_events.set_level(level);
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Drag-and-drop reordering within the parent dice list.
        setup_drag_reorder(&container);

        Self {
            container,
            dice_3d,
            dice_3d_frame,
            stability_label: stability_label.clone(),
            tap_indicator,
            tap_controls,
            led_controls,
            battery_indicator,
            dice_type_selector,
            roll_history,
            compact_box,
        }
    }

    /// Returns the compact mode widget for packing.
    pub fn compact_widget(&self) -> &gtk4::Box {
        &self.compact_box
    }

    /// Apply settings to control visibility of UI elements.
    pub fn apply_settings(&self, settings: &AppSettingsData) {
        self.dice_3d_frame.set_visible(settings.show_dice_3d);
        self.dice_3d.set_rotation_enabled(settings.rotate_dice_3d);
        self.stability_label.set_visible(settings.show_stability_indicator);
        self.tap_indicator.set_visible(settings.show_tap_controls);
        self.tap_controls.set_visible(settings.show_tap_controls);
        self.led_controls.set_visible(settings.show_led_controls);
        self.battery_indicator.set_visible(settings.show_battery_indicator);
        self.dice_type_selector.set_visible(settings.show_dice_type_selector);
        self.roll_history.set_visible(settings.show_roll_history);

        if settings.compact_mode {
            self.container.set_visible(false);
            self.compact_box.set_visible(true);
        } else {
            self.container.set_visible(true);
            self.compact_box.set_visible(false);
        }
    }
}

impl WidgetContainer for DiceRow {
    fn widget(&self) -> &gtk4::Widget {
        self.container.as_ref()
    }
}
