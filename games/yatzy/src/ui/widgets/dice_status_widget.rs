use crate::i18n;
use crate::models::dice_slot::DiceSlot;
use crate::ui::widgets::dice_view::color_css_class;
use dice_rs::DiceColor;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Widget showing the connection status of all 5 dice slots.
///
/// Each dice is displayed as a square colored indicator. Discovered but
/// not-yet-connected dice pulse with a faded color. Connected dice show
/// full color and optional battery level. Clicking a connected dice
/// opens a context menu with a "Swap" option.
pub struct DiceStatusWidget {
    /// The root container widget.
    container: gtk4::Box,
    /// One slot box per dice slot.
    slot_boxes: Vec<gtk4::Box>,
    /// One battery label per dice slot.
    battery_labels: Vec<gtk4::Label>,
    /// One face value label per dice slot.
    face_labels: Vec<gtk4::Label>,
    /// Callback invoked when the user selects "Swap" for a slot.
    swap_callback: Rc<RefCell<Option<Box<dyn Fn(DiceSlot)>>>>,
}

impl DiceStatusWidget {
    /// Create a new dice status widget with 5 empty slots.
    pub fn new() -> Self {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["dice-status"])
            .spacing(12)
            .halign(gtk4::Align::Center)
            .build();

        let swap_callback: Rc<RefCell<Option<Box<dyn Fn(DiceSlot)>>>> = Rc::new(RefCell::new(None));

        let mut slot_boxes = Vec::with_capacity(DiceSlot::COUNT);
        let mut battery_labels = Vec::with_capacity(DiceSlot::COUNT);
        let mut face_labels = Vec::with_capacity(DiceSlot::COUNT);

        for slot in DiceSlot::all() {
            let battery_label = gtk4::Label::builder().css_classes(vec!["dice-battery"]).halign(gtk4::Align::Center).build();
            let face_label = gtk4::Label::builder().css_classes(vec!["dice-face-value"]).halign(gtk4::Align::Center).build();

            let slot_box = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .css_classes(vec!["dice-status-slot"])
                .spacing(2)
                .halign(gtk4::Align::Center)
                .build();

            slot_box.append(&face_label);
            slot_box.append(&battery_label);

            // Add click gesture for swap context menu
            {
                let swap_cb = swap_callback.clone();
                let slot_box_clone = slot_box.clone();
                let gesture = gtk4::GestureClick::new();
                gesture.connect_released(move |_gesture, _n_press, _x, _y| {
                    // Only handle single clicks on connected slots
                    if !slot_box_clone.has_css_class("connected") {
                        return;
                    }
                    // Find the slot index from the slot_boxes vector
                    let idx = slot.get() as usize;
                    let _ = idx; // already known from closure
                    if let Some(ref callback) = *swap_cb.borrow() {
                        callback(slot);
                    }
                });
                slot_box.add_controller(gesture);
            }

            container.append(&slot_box);
            slot_boxes.push(slot_box);
            battery_labels.push(battery_label);
            face_labels.push(face_label);
        }

        Self {
            container,
            slot_boxes,
            battery_labels,
            face_labels,
            swap_callback,
        }
    }

    /// Set the callback invoked when the user clicks a connected dice slot.
    ///
    /// The callback receives the slot that was clicked. The parent widget
    /// is responsible for performing the actual swap (disconnect + rescan).
    pub fn set_swap_callback(&self, callback: Box<dyn Fn(DiceSlot)>) {
        *self.swap_callback.borrow_mut() = Some(callback);
    }

    /// Show a discovered dice (not yet connected) with a faded, pulsing color.
    pub fn set_discovered(&self, slot: DiceSlot, color: DiceColor) {
        let idx = slot.get() as usize;
        self.clear_color_classes(idx);
        self.slot_boxes[idx].add_css_class(color_css_class(color));
        self.slot_boxes[idx].add_css_class("discovered");
        self.slot_boxes[idx].remove_css_class("connected");
        self.battery_labels[idx].set_label("");
        self.face_labels[idx].set_label("");
    }

    /// Mark a slot as connected with the given device name and color.
    pub fn set_connected(&self, slot: DiceSlot, _name: &str, color: DiceColor) {
        let idx = slot.get() as usize;
        self.clear_color_classes(idx);
        self.slot_boxes[idx].add_css_class(color_css_class(color));
        self.slot_boxes[idx].add_css_class("connected");
        self.slot_boxes[idx].remove_css_class("discovered");
        self.slot_boxes[idx].remove_css_class("swapping");
        self.slot_boxes[idx].set_tooltip_text(Some(&i18n::get("dice-swap-tooltip")));
    }

    /// Set the battery level for a slot.
    pub fn set_battery(&self, slot: DiceSlot, level: u8) {
        let idx = slot.get() as usize;
        self.battery_labels[idx].set_label(&format!("{level}%"));
        if level <= 20 {
            self.battery_labels[idx].add_css_class("battery-low");
        } else {
            self.battery_labels[idx].remove_css_class("battery-low");
        }
    }

    /// Mark a slot as disconnected.
    pub fn set_disconnected(&self, slot: DiceSlot) {
        let idx = slot.get() as usize;
        self.clear_color_classes(idx);
        self.slot_boxes[idx].remove_css_class("connected");
        self.slot_boxes[idx].remove_css_class("discovered");
        self.slot_boxes[idx].remove_css_class("swapping");
        self.battery_labels[idx].set_label("");
        self.face_labels[idx].set_label("");
    }

    /// Mark a slot as being swapped (disconnected, searching for replacement).
    pub fn set_swapping(&self, slot: DiceSlot) {
        let idx = slot.get() as usize;
        self.clear_color_classes(idx);
        self.slot_boxes[idx].remove_css_class("connected");
        self.slot_boxes[idx].remove_css_class("discovered");
        self.slot_boxes[idx].add_css_class("swapping");
        self.battery_labels[idx].set_label("");
        self.face_labels[idx].set_label("");
    }

    /// Set the face value for a slot (shown on the setup screen).
    pub fn set_face_value(&self, slot: DiceSlot, face: dice_rs::FaceValue) {
        let idx = slot.get() as usize;
        self.face_labels[idx].set_label(&face.get().to_string());
    }

    /// Reset all slots to disconnected state.
    pub fn reset(&self) {
        for slot in DiceSlot::all() {
            self.set_disconnected(slot);
        }
    }

    /// Find the first slot that has no color assigned (neither discovered nor connected).
    pub fn next_empty_slot(&self) -> Option<DiceSlot> {
        for (idx, slot_box) in self.slot_boxes.iter().enumerate() {
            let has_color = ["dice-black", "dice-red", "dice-green", "dice-blue", "dice-yellow", "dice-orange"]
                .iter()
                .any(|c| slot_box.has_css_class(c));
            if !has_color {
                return DiceSlot::new(idx as u8).ok();
            }
        }
        None
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }

    /// Remove all color CSS classes from a slot.
    fn clear_color_classes(&self, idx: usize) {
        for c in ["dice-black", "dice-red", "dice-green", "dice-blue", "dice-yellow", "dice-orange"] {
            self.slot_boxes[idx].remove_css_class(c);
        }
    }
}

impl Default for DiceStatusWidget {
    fn default() -> Self {
        Self::new()
    }
}
