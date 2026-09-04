use crate::i18n;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use dice_rs::DiceColor;
use dice_rs::FaceValue;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Callback type for when a die's hold state is toggled.
type HoldToggledCallback = Rc<dyn Fn(DiceSlot, bool)>;

/// Visual representation of the 5 dice with hold toggles.
///
/// Each die is shown as a clickable button displaying its face value.
/// Clicking a die toggles its hold state. Held dice are highlighted
/// with a green border. Rolling dice show an animated state.
pub struct DiceView {
    /// The root container widget.
    container: gtk4::Box,
    /// The 5 die buttons.
    die_buttons: Vec<gtk4::Button>,
    /// The 5 die labels (for face value display).
    die_labels: Vec<gtk4::Label>,
    /// Current hold mask.
    holds: Rc<RefCell<HoldMask>>,
    /// Callback invoked when a die's hold state is toggled.
    on_hold_toggled: Rc<RefCell<Option<HoldToggledCallback>>>,
}

impl DiceView {
    /// Create a new dice view with 5 die buttons.
    pub fn new() -> Self {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["dice-view"])
            .spacing(12)
            .halign(gtk4::Align::Center)
            .build();

        let holds = Rc::new(RefCell::new(HoldMask::none()));
        let on_hold_toggled: Rc<RefCell<Option<HoldToggledCallback>>> = Rc::new(RefCell::new(None));

        let mut die_buttons = Vec::with_capacity(DiceSlot::COUNT);
        let mut die_labels = Vec::with_capacity(DiceSlot::COUNT);

        for slot in DiceSlot::all() {
            let label = gtk4::Label::builder().label("?").css_classes(vec!["die-label"]).build();

            let button = gtk4::Button::builder()
                .css_classes(vec!["die-button"])
                .child(&label)
                .tooltip_text(i18n::get_int("dice-tooltip-hold", "slot", (slot.get() + 1) as i64))
                .build();

            // Click handler for hold toggle
            let holds_clone = holds.clone();
            let on_toggled = on_hold_toggled.clone();
            let slot_for_click = slot;
            let button_for_closure = button.clone();
            button.connect_clicked(move |_| {
                let new_held = {
                    let mut mask = holds_clone.borrow_mut();
                    let new_held = !mask.is_held(slot_for_click);
                    mask.toggle(slot_for_click);
                    new_held
                };
                // Update visual state immediately
                if new_held {
                    button_for_closure.add_css_class("held");
                } else {
                    button_for_closure.remove_css_class("held");
                }
                if let Some(callback) = on_toggled.borrow().as_ref() {
                    callback(slot_for_click, new_held);
                }
            });

            container.append(&button);
            die_buttons.push(button);
            die_labels.push(label);
        }

        Self {
            container,
            die_buttons,
            die_labels,
            holds,
            on_hold_toggled,
        }
    }

    /// Set the callback invoked when a die's hold state is toggled.
    pub fn connect_hold_toggled<F>(&self, callback: F)
    where
        F: Fn(DiceSlot, bool) + 'static,
    {
        *self.on_hold_toggled.borrow_mut() = Some(Rc::new(callback));
    }

    /// Update the face values displayed on the dice.
    pub fn update_faces(&self, faces: &[Option<FaceValue>; 5]) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            if let Some(face) = faces[idx] {
                self.die_labels[idx].set_label(&face.get().to_string());
            } else {
                self.die_labels[idx].set_label("?");
            }
        }
    }

    /// Get the currently displayed face values.
    ///
    /// Parses the label text of each die button back into `FaceValue`s.
    /// Returns `None` for slots showing "?" or "..." (not yet rolled).
    pub fn current_faces(&self) -> [Option<FaceValue>; 5] {
        let mut result = [None; 5];
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            let text = self.die_labels[idx].label();
            if let Ok(value) = text.parse::<u8>()
                && let Ok(face) = FaceValue::new(value)
            {
                result[idx] = Some(face);
            }
        }
        result
    }

    /// Update the hold mask and refresh visual state.
    pub fn update_holds(&self, holds: HoldMask) {
        *self.holds.borrow_mut() = holds;
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            let button = &self.die_buttons[idx];
            if holds.is_held(slot) {
                button.add_css_class("held");
            } else {
                button.remove_css_class("held");
            }
        }
    }

    /// Set the rolling animation state for all dice.
    pub fn set_rolling(&self) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            let button = &self.die_buttons[idx];
            button.add_css_class("rolling");
            self.die_labels[idx].set_label("...");
        }
    }

    /// Set the rolling animation only for non-held dice.
    /// Held dice keep their face value and visual state.
    pub fn set_rolling_with_holds(&self, holds: HoldMask, current_faces: [Option<FaceValue>; 5]) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            if holds.is_held(slot) {
                // Clear rolling state from held dice, restore their face label
                self.die_buttons[idx].remove_css_class("rolling");
                if let Some(face) = current_faces[idx] {
                    self.die_labels[idx].set_label(&face.to_string());
                }
            } else {
                self.die_buttons[idx].add_css_class("rolling");
                self.die_labels[idx].set_label("...");
            }
        }
    }

    /// Clear the rolling animation state.
    pub fn clear_rolling(&self) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            self.die_buttons[idx].remove_css_class("rolling");
        }
    }

    /// Mark a specific die as disconnected.
    pub fn set_disconnected(&self, slot: DiceSlot) {
        let idx = slot.get() as usize;
        let button = &self.die_buttons[idx];
        button.add_css_class("disconnected");
        self.die_labels[idx].set_label("-");
    }

    /// Clear the disconnected state for a die.
    pub fn clear_disconnected(&self, slot: DiceSlot) {
        let idx = slot.get() as usize;
        self.die_buttons[idx].remove_css_class("disconnected");
    }

    /// Reset all dice to their default state.
    pub fn reset(&self) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            let button = &self.die_buttons[idx];
            button.remove_css_class("held");
            button.remove_css_class("rolling");
            button.remove_css_class("disconnected");
            self.die_labels[idx].set_label("?");
        }
        *self.holds.borrow_mut() = HoldMask::none();
    }

    /// Set the color indicator for a specific dice slot.
    ///
    /// Adds a CSS class based on the physical dice color so the player
    /// can match physical dice to UI buttons.
    pub fn set_slot_color(&self, slot: DiceSlot, color: DiceColor) {
        let idx = slot.get() as usize;
        let button = &self.die_buttons[idx];
        // Remove any existing color classes
        for c in ["dice-black", "dice-red", "dice-green", "dice-blue", "dice-yellow", "dice-orange"] {
            button.remove_css_class(c);
        }
        let css_class = color_css_class(color);
        button.add_css_class(css_class);
    }

    /// Clear all color indicators from dice buttons.
    pub fn clear_slot_colors(&self) {
        for slot in DiceSlot::all() {
            let idx = slot.get() as usize;
            let button = &self.die_buttons[idx];
            for c in ["dice-black", "dice-red", "dice-green", "dice-blue", "dice-yellow", "dice-orange"] {
                button.remove_css_class(c);
            }
        }
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for DiceView {
    fn default() -> Self {
        Self::new()
    }
}

/// Map a `DiceColor` to a CSS class name for dice button styling.
pub fn color_css_class(color: DiceColor) -> &'static str {
    match color {
        DiceColor::Black => "dice-black",
        DiceColor::Red => "dice-red",
        DiceColor::Green => "dice-green",
        DiceColor::Blue => "dice-blue",
        DiceColor::Yellow => "dice-yellow",
        DiceColor::Orange => "dice-orange",
    }
}
