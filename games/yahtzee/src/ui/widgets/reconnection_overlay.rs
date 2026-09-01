use crate::models::dice_slot::DiceSlot;
use gtk4::prelude::*;

/// Overlay widget shown when a dice disconnects mid-game.
///
/// Displays a warning message, the disconnected slot number, and a
/// "Retry" button to attempt reconnection. The overlay is modal —
/// it blocks game interaction until the dice is reconnected or
/// the user dismisses it.
pub struct ReconnectionOverlay {
    /// The root container widget.
    container: gtk4::Box,
    /// Label showing the warning message.
    #[allow(dead_code)]
    message_label: gtk4::Label,
    /// Label showing which slot(s) are disconnected.
    slot_label: gtk4::Label,
    /// The "retry" button.
    retry_button: gtk4::Button,
    /// The "dismiss" button.
    dismiss_button: gtk4::Button,
}

impl ReconnectionOverlay {
    /// Create a new reconnection overlay.
    pub fn new() -> Self {
        let message_label = gtk4::Label::builder()
            .label("Würfel-Verbindung verloren!")
            .css_classes(vec!["reconnection-message"])
            .halign(gtk4::Align::Center)
            .build();

        let slot_label = gtk4::Label::builder()
            .css_classes(vec!["reconnection-slot"])
            .halign(gtk4::Align::Center)
            .build();

        let retry_button = gtk4::Button::builder()
            .label("Erneut verbinden")
            .css_classes(vec!["reconnection-retry-button", "suggested-action"])
            .halign(gtk4::Align::Center)
            .build();

        let dismiss_button = gtk4::Button::builder()
            .label("Ignorieren")
            .css_classes(vec!["reconnection-dismiss-button"])
            .halign(gtk4::Align::Center)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["reconnection-overlay"])
            .spacing(16)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        container.append(&message_label);
        container.append(&slot_label);
        container.append(&retry_button);
        container.append(&dismiss_button);

        Self {
            container,
            message_label,
            slot_label,
            retry_button,
            dismiss_button,
        }
    }

    /// Update the overlay with the disconnected slot information.
    pub fn show_disconnected(&self, slot: DiceSlot) {
        let slot_text = format!("Würfel {} ist getrennt", slot.get() + 1);
        self.slot_label.set_label(&slot_text);
    }

    /// Update the overlay with multiple disconnected slots.
    pub fn show_multiple_disconnected(&self, slots: &[DiceSlot]) {
        if slots.is_empty() {
            self.clear();
            return;
        }
        if slots.len() == 1 {
            self.show_disconnected(slots[0]);
            return;
        }
        let slot_numbers: Vec<String> = slots.iter().map(|s| format!("{}", s.get() + 1)).collect();
        let slot_text = format!("Würfel {} sind getrennt", slot_numbers.join(", "));
        self.slot_label.set_label(&slot_text);
    }

    /// Get the "retry" button (for connecting signals).
    pub fn retry_button(&self) -> &gtk4::Button {
        &self.retry_button
    }

    /// Get the "dismiss" button (for connecting signals).
    pub fn dismiss_button(&self) -> &gtk4::Button {
        &self.dismiss_button
    }

    /// Clear the overlay.
    pub fn clear(&self) {
        self.slot_label.set_label("");
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for ReconnectionOverlay {
    fn default() -> Self {
        Self::new()
    }
}
