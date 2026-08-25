use std::sync::Arc;

use dice_rs::model::dice::DiceColor;
use dice_rs::service::dice::Dice;
use dice_rs::service::manager::DiceManager;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;

use crate::battery_indicator::BatteryIndicator;
use crate::dice_3d::Dice3D;
use crate::event_controller::EventController;
use crate::face_display::FaceDisplay;
use crate::face_display::RollHistory;
use crate::led_controls::LedControls;
use crate::tap_indicator::TapIndicator;

/// Map a DiceColor to a CSS class name for the border.
fn dice_color_to_css_class(color: DiceColor) -> &'static str {
    match color {
        DiceColor::Black => "dice-border-black",
        DiceColor::Red => "dice-border-red",
        DiceColor::Green => "dice-border-green",
        DiceColor::Blue => "dice-border-blue",
        DiceColor::Yellow => "dice-border-yellow",
        DiceColor::Orange => "dice-border-orange",
    }
}

/// A list row representing a single connected dice.
pub struct DiceRow {
    container: gtk4::Box,
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
        led_controls.set_dice(dice.clone());

        // Left side: 3D dice view.
        let dice_3d_frame = gtk4::Frame::builder()
            .css_classes(vec!["dice-3d-frame"])
            .width_request(120)
            .height_request(120)
            .child(dice_3d.widget())
            .build();

        // Right side: face value, stability, battery, history, LED controls.
        let info_box = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).hexpand(true).build();

        let header = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
        header.append(face_display.widget());
        header.append(face_display.stability_label());
        header.append(tap_indicator.widget());

        let battery_row = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).build();
        battery_row.append(battery_indicator.label());
        let level_bar = battery_indicator.level_bar();
        level_bar.set_hexpand(true);
        level_bar.set_valign(gtk4::Align::Center);
        level_bar.add_css_class("battery-level-bar");
        battery_row.append(level_bar);

        info_box.append(&header);
        info_box.append(roll_history.widget());
        info_box.append(led_controls.widget());
        info_box.append(&battery_row);

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .css_classes(vec!["dice-row"])
            .build();
        container.append(&dice_3d_frame);
        container.append(&info_box);

        // Apply colored border based on dice physical color.
        let container_clone = container.clone();
        let dice_for_color = dice.clone();
        let face_display_for_color = face_display.clone();
        let roll_history_for_color = roll_history.clone();
        let dice_3d_for_color = dice_3d.clone();
        glib::spawn_future_local(async move {
            if let Ok(color) = dice_for_color.get_color().await {
                container_clone.add_css_class(dice_color_to_css_class(color));
                face_display_for_color.set_dice_color(color);
                roll_history_for_color.set_dice_color(color);
                dice_3d_for_color.set_color(color);
            }
        });

        let controller = EventController::new(dice, manager, face_display, battery_indicator, dice_3d, roll_history, tap_indicator);
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

        Self { container }
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }
}
