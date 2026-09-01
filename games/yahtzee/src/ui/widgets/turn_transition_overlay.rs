use crate::models::turn_transition::TurnTransition;
use gtk4::prelude::*;

/// Overlay widget shown during pass-and-play turn transitions.
///
/// Displays the next player's name, color indicator, and round number.
/// The overlay is modal — it blocks interaction until the next player
/// acknowledges by pressing "Bereit" (Ready).
pub struct TurnTransitionOverlay {
    /// The root container widget.
    container: gtk4::Box,
    /// Label showing the round number.
    round_label: gtk4::Label,
    /// Label showing the next player's name.
    name_label: gtk4::Label,
    /// The color indicator box.
    color_box: gtk4::Box,
    /// The "ready" button.
    ready_button: gtk4::Button,
}

impl TurnTransitionOverlay {
    /// Create a new turn transition overlay.
    pub fn new() -> Self {
        let round_label = gtk4::Label::builder().css_classes(vec!["transition-round"]).halign(gtk4::Align::Center).build();

        let name_label = gtk4::Label::builder()
            .css_classes(vec!["transition-player-name"])
            .halign(gtk4::Align::Center)
            .build();

        let color_box = gtk4::Box::builder()
            .css_classes(vec!["transition-color-indicator"])
            .halign(gtk4::Align::Center)
            .width_request(48)
            .height_request(48)
            .build();

        let ready_button = gtk4::Button::builder()
            .label("Bereit!")
            .css_classes(vec!["transition-ready-button", "suggested-action"])
            .halign(gtk4::Align::Center)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["turn-transition-overlay"])
            .spacing(16)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        container.append(&round_label);
        container.append(&color_box);
        container.append(&name_label);
        container.append(&ready_button);

        Self {
            container,
            round_label,
            name_label,
            color_box,
            ready_button,
        }
    }

    /// Update the overlay with turn transition data.
    pub fn update(&self, transition: &TurnTransition) {
        let round_text = format!("Runde {}", transition.round());
        self.round_label.set_label(&round_text);

        let name_text = format!("{} ist dran!", transition.player_name());
        self.name_label.set_label(&name_text);

        // Apply color via CSS class based on the player's color
        let color_class = color_to_css_class(transition.player_color());
        // Remove all color classes and add the new one
        for c in ["color-red", "color-green", "color-blue", "color-yellow", "color-orange", "color-purple"] {
            self.color_box.remove_css_class(c);
        }
        self.color_box.add_css_class(&color_class);
    }

    /// Get the "ready" button (for connecting signals).
    pub fn ready_button(&self) -> &gtk4::Button {
        &self.ready_button
    }

    /// Clear the overlay.
    pub fn clear(&self) {
        self.round_label.set_label("");
        self.name_label.set_label("");
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for TurnTransitionOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// Map a `PlayerColor` to a CSS class name for the color indicator.
fn color_to_css_class(color: crate::models::player_color::PlayerColor) -> String {
    use crate::models::player_color::PlayerColor;
    if color == PlayerColor::RED {
        "color-red"
    } else if color == PlayerColor::GREEN {
        "color-green"
    } else if color == PlayerColor::BLUE {
        "color-blue"
    } else if color == PlayerColor::YELLOW {
        "color-yellow"
    } else if color == PlayerColor::ORANGE {
        "color-orange"
    } else if color == PlayerColor::PURPLE {
        "color-purple"
    } else {
        "color-white"
    }
    .to_string()
}
