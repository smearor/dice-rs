use crate::i18n;
use crate::models::roll_count::RollCount;
use crate::ui::models::roll_button_label::RollButtonLabel;
use crate::ui::models::ui_message::UiMessage;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Callback type for when the roll button is clicked.
type RollCallback = Rc<dyn Fn()>;

/// Widget displaying turn instructions and the roll button.
///
/// Shows the current turn instruction message, the roll count,
/// and a button to initiate rolling. The button label and
/// sensitivity change based on the current game state.
pub struct TurnPanel {
    /// The root container widget.
    container: gtk4::Box,
    /// The instruction label.
    instruction_label: gtk4::Label,
    /// The roll count label.
    roll_count_label: gtk4::Label,
    /// The roll button.
    roll_button: gtk4::Button,
    /// Callback invoked when the roll button is clicked.
    on_roll: Rc<RefCell<Option<RollCallback>>>,
}

impl TurnPanel {
    /// Create a new turn panel.
    pub fn new() -> Self {
        let instruction_label = gtk4::Label::builder()
            .css_classes(vec!["turn-instruction"])
            .halign(gtk4::Align::Center)
            .hexpand(true)
            .build();

        let roll_count_label = gtk4::Label::builder().css_classes(vec!["roll-counter"]).halign(gtk4::Align::Center).build();

        let roll_button = gtk4::Button::builder()
            .css_classes(vec!["roll-button", "suggested-action"])
            .label(&RollButtonLabel::Wuerfeln.text())
            .halign(gtk4::Align::Center)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["turn-panel"])
            .spacing(8)
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        container.append(&instruction_label);
        container.append(&roll_count_label);
        container.append(&roll_button);

        let on_roll = Rc::new(RefCell::new(None::<RollCallback>));

        // Connect roll button click
        {
            let on_roll_clone = on_roll.clone();
            roll_button.connect_clicked(move |_| {
                if let Some(callback) = on_roll_clone.borrow().as_ref() {
                    callback();
                }
            });
        }

        Self {
            container,
            instruction_label,
            roll_count_label,
            roll_button,
            on_roll,
        }
    }

    /// Set the callback invoked when the roll button is clicked.
    pub fn connect_roll<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        *self.on_roll.borrow_mut() = Some(Rc::new(callback));
    }

    /// Update the instruction message.
    pub fn set_message(&self, message: &UiMessage) {
        self.instruction_label.set_label(message.as_str());
    }

    /// Update the roll button label and sensitivity.
    pub fn set_roll_button_label(&self, label: RollButtonLabel) {
        self.roll_button.set_label(&label.text());
        self.roll_button.set_sensitive(label.is_sensitive());
    }

    /// Update the roll count display.
    pub fn set_roll_count(&self, count: RollCount) {
        let text = i18n::get_int("roll-count", "current", count.get() as i64);
        self.roll_count_label.set_label(&text);
    }

    /// Set the panel to the rolling state.
    pub fn set_rolling(&self) {
        self.set_roll_button_label(RollButtonLabel::WurfLauft);
    }

    /// Set the panel to the waiting state after a roll.
    pub fn set_roll_complete(&self, count: RollCount) {
        let label = RollButtonLabel::from_roll_count(count.get());
        self.set_roll_button_label(label);
        let remaining = 3 - count.get();
        let text = i18n::get_int_int("roll-count-remaining", "current", count.get() as i64, "remaining", remaining as i64);
        self.roll_count_label.set_label(&text);
    }

    /// Set the panel to the game-over state.
    pub fn set_game_over(&self) {
        self.set_roll_button_label(RollButtonLabel::SpielBeendet);
        self.roll_count_label.set_label("");
    }

    /// Reset the panel to the initial state.
    pub fn reset(&self) {
        self.set_roll_button_label(RollButtonLabel::Wuerfeln);
        self.roll_count_label.set_label(&i18n::get("roll-count-zero"));
        self.instruction_label.set_label("");
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for TurnPanel {
    fn default() -> Self {
        Self::new()
    }
}
