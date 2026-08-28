use std::sync::Arc;

use std::str::FromStr;

use dice_rs::model::dice::DiceType;
use dice_rs::service::dice::Dice;
use dice_rs::service::manager::DiceManager;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use tracing::debug;

use crate::app_settings::AppSettingsData;
use crate::battery_indicator::BatteryIndicator;
use crate::dice_3d::Dice3D;
use crate::dice_style::DiceColorStyle;
use crate::dice_type_icon::create_icon;
use crate::event_controller::EventController;
use crate::face_display::FaceDisplay;
use crate::led_controls::LedControls;
use crate::roll_history::RollHistory;
use crate::tap_controls::TapControls;
use crate::tap_indicator::TapIndicator;

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
    /// The container holding the LED control widgets.
    led_controls_widget: gtk4::Box,
    /// Horizontal box with battery label and level bar.
    battery_row: gtk4::Box,
    /// Dropdown for selecting the dice shell type (D6, D20, etc.).
    dice_type_selector: gtk4::DropDown,
    /// The container holding the roll history widget.
    roll_history_widget: gtk4::Box,
    /// Compact single-line row shown when compact mode is enabled.
    compact_box: gtk4::Box,
}

impl DiceRow {
    /// Create a new dice row for a connected dice.
    pub fn new(dice: Dice, manager: Arc<DiceManager>) -> Self {
        let face_display = FaceDisplay::new();
        let battery_indicator = BatteryIndicator::new();
        let led_controls = LedControls::new();
        let dice_3d = Dice3D::new();
        let roll_history = RollHistory::new();
        let tap_indicator = TapIndicator::new();
        let tap_controls = TapControls::new();
        led_controls.set_dice(dice.clone());
        tap_controls.set_dice(dice.clone());

        // Load per-dice settings from disk if available.
        let device_name = dice.name().to_string();
        led_controls.set_device_name(device_name.clone());
        let dice_settings = crate::config_dir::load_dice_settings(&device_name);
        if let Some(ref settings) = dice_settings {
            dice.set_dice_type(settings.dice_type);
            dice_3d.set_dice_type(settings.dice_type);
            led_controls.set_colors(settings.led_color1, settings.led_color2);
        }

        // Left side: 3D dice view.
        let dice_3d_frame = gtk4::Frame::builder()
            .css_classes(vec!["dice-3d-frame"])
            .width_request(120)
            .height_request(120)
            .child(dice_3d.widget())
            .build();

        // Right side: face value, stability, battery, history, LED controls.
        let info_box = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).hexpand(true).build();

        let dice_types = DiceType::sorted_by_count();
        let dice_type_labels: Vec<String> = dice_types.iter().map(|t| t.to_string()).collect();
        let dice_type_model = gtk4::StringList::new(&dice_type_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let dice_type_selector = gtk4::DropDown::builder()
            .model(&dice_type_model)
            .tooltip_text("Dice shell type")
            .css_classes(vec!["dice-type-selector"])
            .build();

        // Custom factory: render isometric dice icons in dropdown items.
        let factory = gtk4::SignalListItemFactory::new();
        let types_for_setup = dice_types.clone();
        factory.connect_setup(move |_item, list_item| {
            let overlay = gtk4::Overlay::builder().css_classes(vec!["dice-type-item"]).build();
            list_item.set_child(Some(&overlay));
        });
        factory.connect_bind(move |_item, list_item| {
            let position = list_item.position() as usize;
            let dice_type = types_for_setup[position % types_for_setup.len()];
            let icon = create_icon(dice_type);
            icon.set_hexpand(true);
            icon.set_vexpand(true);
            icon.set_halign(gtk4::Align::Center);
            icon.set_valign(gtk4::Align::Center);
            let label = gtk4::Label::builder()
                .label(dice_type.to_string())
                .css_classes(vec!["dice-type-label"])
                .halign(gtk4::Align::Center)
                .valign(gtk4::Align::Center)
                .build();
            let overlay = list_item.child().and_downcast::<gtk4::Overlay>().expect("child is Overlay");
            overlay.set_child(Some(&icon));
            overlay.add_overlay(&label);
        });
        dice_type_selector.set_factory(Some(&factory));

        // Square size matching face display.
        dice_type_selector.set_size_request(80, 80);
        dice_type_selector.set_margin_top(8);
        dice_type_selector.set_margin_end(8);

        // Select the current dice type.
        let current_type = dice.dice_type();
        if let Some(pos) = dice_types.iter().position(|t| *t == current_type) {
            dice_type_selector.set_selected(pos as u32);
        }

        let dice_for_type = dice.clone();
        let dice_3d_for_type = dice_3d.clone();
        let device_name_for_type = device_name.clone();
        dice_type_selector.connect_notify_local(Some("selected"), move |dropdown, _pspec| {
            let Some(item) = dropdown.selected_item() else {
                return;
            };
            let Some(text) = item.downcast::<gtk4::StringObject>().ok() else {
                return;
            };
            match DiceType::from_str(text.string().as_str()) {
                Ok(dt) => {
                    dice_for_type.set_dice_type(dt);
                    dice_3d_for_type.set_dice_type(dt);
                    let mut settings = crate::config_dir::load_dice_settings(&device_name_for_type).unwrap_or_default();
                    settings.dice_type = dt;
                    crate::config_dir::save_dice_settings(&device_name_for_type, &settings);
                }
                Err(error) => debug!(error = %error, "invalid dice type selected"),
            }
        });

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

        let battery_row = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).build();
        battery_row.append(battery_indicator.label());
        let level_bar = battery_indicator.level_bar();
        level_bar.set_hexpand(true);
        level_bar.set_valign(gtk4::Align::Center);
        level_bar.add_css_class("battery-level-bar");
        battery_row.append(level_bar);

        let roll_history_widget = roll_history.widget().clone();
        let led_controls_widget = led_controls.widget().clone();

        // Vertical box combining roll history and battery indicator.
        let history_battery_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(4)
            .valign(gtk4::Align::Center)
            .hexpand(true)
            .build();
        history_battery_box.append(&roll_history_widget);
        history_battery_box.append(&battery_row);
        header.append(&history_battery_box);

        let tap_widget = tap_indicator.widget();
        tap_widget.set_valign(gtk4::Align::Center);
        header.append(tap_widget);

        dice_type_selector.set_valign(gtk4::Align::Center);
        header.append(&dice_type_selector);

        info_box.append(&header);

        let controls_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_bottom(8)
            .build();
        controls_row.append(led_controls.widget());
        tap_controls.widget().set_hexpand(true);
        tap_controls.widget().set_halign(gtk4::Align::End);
        controls_row.append(tap_controls.widget());
        info_box.append(&controls_row);

        // Compact mode: single-line row with face value and battery.
        let compact_label = gtk4::Label::builder()
            .label("?")
            .css_classes(vec!["compact-face"])
            .halign(gtk4::Align::Start)
            .build();
        let compact_battery_label = gtk4::Label::builder()
            .label("N/A")
            .css_classes(vec!["compact-battery"])
            .halign(gtk4::Align::End)
            .hexpand(true)
            .build();
        let compact_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .css_classes(vec!["dice-row", "compact-row"])
            .build();
        compact_box.append(&compact_label);
        compact_box.append(&compact_battery_label);

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
        let dice_for_color = dice.clone();
        let face_display_for_color = face_display.clone();
        let roll_history_for_color = roll_history.clone();
        let dice_3d_for_color = dice_3d.clone();
        let compact_battery_for_color = compact_battery_label.clone();
        glib::spawn_future_local(async move {
            if let Ok(color) = dice_for_color.get_color().await {
                let style = DiceColorStyle::from(color);
                container_clone.add_css_class(style.border_css_class());
                compact_box_clone.add_css_class(style.border_css_class());
                face_display_for_color.set_dice_color(color);
                roll_history_for_color.set_dice_color(color);
                dice_3d_for_color.set_color(color);
            }
            if let Ok(level) = dice_for_color.get_battery_level().await {
                compact_battery_for_color.set_label(&format!("{}%", level.get()));
            }
        });

        let controller = EventController::new(
            dice,
            manager,
            face_display.clone(),
            battery_indicator,
            dice_3d.clone(),
            roll_history,
            tap_indicator.clone(),
            Some(compact_label.clone()),
        );
        controller.start();

        // --- Drag-and-drop reordering ---

        // DragSource: provides the container's index within its parent Box.
        let drag_source = gtk4::DragSource::builder().actions(gdk::DragAction::MOVE).build();

        let drag_container = container.clone();
        drag_source.connect_prepare(move |_source, _x, _y| {
            let parent = drag_container.parent()?;
            let parent = parent.downcast::<gtk4::Box>().ok()?;
            let mut index = 0i32;
            let mut child = parent.first_child();
            while let Some(c) = child {
                if c == drag_container {
                    let value = glib::Value::from(index);
                    return Some(gdk::ContentProvider::for_value(&value));
                }
                index += 1;
                child = c.next_sibling();
            }
            None
        });

        container.add_controller(drag_source);

        // DropTarget: accepts a source index and reorders within the parent Box.
        let drop_target = gtk4::DropTarget::new(glib::Type::I32, gdk::DragAction::MOVE);

        let drop_container = container.clone();
        drop_target.connect_motion(|_target, _x, _y| gdk::DragAction::MOVE);

        drop_target.connect_drop(move |_target, value, _x, _y| {
            let Ok(source_index) = value.get::<i32>() else {
                return false;
            };

            let Some(parent) = drop_container.parent().and_then(|p| p.downcast::<gtk4::Box>().ok()) else {
                return false;
            };

            // Find target index by iterating siblings.
            let mut target_index = 0i32;
            let mut child = parent.first_child();
            let mut found = false;
            while let Some(c) = child {
                if c == drop_container {
                    found = true;
                    break;
                }
                target_index += 1;
                child = c.next_sibling();
            }
            if !found || source_index == target_index {
                return false;
            }

            // Defer reorder to idle callback. Use reorder_child_after which
            // moves the widget without remove/append (no reparenting, safe
            // for GLArea/Dice3D widgets).
            let parent = parent.clone();
            glib::idle_add_local_once(move || {
                // Collect all children to find source and target widgets.
                let mut children: Vec<gtk4::Widget> = Vec::new();
                let mut child = parent.first_child();
                while let Some(c) = child {
                    children.push(c.clone());
                    child = c.next_sibling();
                }

                let source = source_index as usize;
                let target = target_index as usize;
                if source >= children.len() || target >= children.len() || source == target {
                    return;
                }

                let source_widget = &children[source];
                if source_index < target_index {
                    // Moving down: insert after the target widget.
                    let sibling = &children[target];
                    parent.reorder_child_after(source_widget, Some(sibling));
                } else {
                    // Moving up: insert after the widget before target,
                    // or prepend if target is 0.
                    if target_index == 0 {
                        parent.reorder_child_after(source_widget, None::<&gtk4::Widget>);
                    } else {
                        let sibling = &children[target - 1];
                        parent.reorder_child_after(source_widget, Some(sibling));
                    }
                }
            });

            true
        });

        container.add_controller(drop_target);

        Self {
            container,
            dice_3d,
            dice_3d_frame,
            stability_label: stability_label.clone(),
            tap_indicator,
            tap_controls,
            led_controls_widget,
            battery_row,
            dice_type_selector,
            roll_history_widget,
            compact_box,
        }
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
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
        self.tap_indicator.widget().set_visible(settings.show_tap_controls);
        self.tap_controls.widget().set_visible(settings.show_tap_controls);
        self.led_controls_widget.set_visible(settings.show_led_controls);
        self.battery_row.set_visible(settings.show_battery_indicator);
        self.dice_type_selector.set_visible(settings.show_dice_type_selector);
        self.roll_history_widget.set_visible(settings.show_roll_history);

        if settings.compact_mode {
            self.container.set_visible(false);
            self.compact_box.set_visible(true);
        } else {
            self.container.set_visible(true);
            self.compact_box.set_visible(false);
        }
    }
}
