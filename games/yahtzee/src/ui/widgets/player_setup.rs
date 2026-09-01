use crate::error::Result;
use crate::models::player_color::PlayerColor;
use crate::models::player_config::PlayerConfig;
use crate::models::player_config::PlayerSetup;
use crate::models::player_name::PlayerName;
use crate::models::player_type::PlayerType;
use gtk4::prelude::*;

/// Widget for configuring players before starting a game.
///
/// Provides UI controls for adding/removing players, editing names,
/// selecting colors, and choosing between human and computer types.
/// When the user is done, call `build_setup()` to get a validated
/// `PlayerSetup` that can be converted into players.
pub struct PlayerSetupWidget {
    /// The root container widget.
    container: gtk4::Box,
    /// The list of player configuration rows.
    rows: Vec<PlayerRow>,
    /// The "add player" button.
    add_button: gtk4::Button,
    /// The "start game" button.
    start_button: gtk4::Button,
}

/// A single player configuration row in the setup screen.
struct PlayerRow {
    /// The container box for this row.
    box_widget: gtk4::Box,
    /// The name entry field.
    name_entry: gtk4::Entry,
    /// The color dropdown.
    color_dropdown: gtk4::DropDown,
    /// The type dropdown (Human/Computer).
    type_dropdown: gtk4::DropDown,
    /// The remove button.
    remove_button: gtk4::Button,
}

impl PlayerRow {
    /// Create a new player row with default values.
    fn new(index: usize) -> Self {
        let name_entry = gtk4::Entry::builder()
            .text(format!("Spieler {}", index + 1))
            .css_classes(vec!["player-name-entry"])
            .hexpand(true)
            .build();

        let color_model = gtk4::StringList::new(&["Rot", "Grün", "Blau", "Gelb", "Orange", "Lila"]);
        let color_dropdown = gtk4::DropDown::builder()
            .model(&color_model)
            .selected(index as u32 % 6)
            .css_classes(vec!["player-color-dropdown"])
            .build();

        let type_model = gtk4::StringList::new(&["Mensch", "Computer"]);
        let type_dropdown = gtk4::DropDown::builder()
            .model(&type_model)
            .selected(0)
            .css_classes(vec!["player-type-dropdown"])
            .build();

        let remove_button = gtk4::Button::builder()
            .label("✕")
            .css_classes(vec!["player-remove-button"])
            .tooltip_text("Spieler entfernen")
            .build();

        let box_widget = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .css_classes(vec!["player-row"])
            .hexpand(true)
            .build();

        box_widget.append(&name_entry);
        box_widget.append(&color_dropdown);
        box_widget.append(&type_dropdown);
        box_widget.append(&remove_button);

        Self {
            box_widget,
            name_entry,
            color_dropdown,
            type_dropdown,
            remove_button,
        }
    }

    /// Get the configured player name from the entry field.
    fn player_name(&self) -> Result<PlayerName> {
        let text = self.name_entry.text().to_string();
        PlayerName::new(text)
    }

    /// Get the configured player color from the dropdown.
    fn player_color(&self) -> PlayerColor {
        let index = self.color_dropdown.selected() as usize;
        PlayerColor::DEFAULTS[index % PlayerColor::DEFAULTS.len()]
    }

    /// Get the configured player type from the dropdown.
    fn player_type(&self) -> PlayerType {
        if self.type_dropdown.selected() == 0 {
            PlayerType::Human
        } else {
            PlayerType::Computer
        }
    }

    /// Build a `PlayerConfig` from this row's current values.
    fn to_config(&self) -> Result<PlayerConfig> {
        Ok(PlayerConfig::new(self.player_name()?, self.player_color(), self.player_type()))
    }

    /// Get the remove button (for connecting signals).
    fn remove_button(&self) -> &gtk4::Button {
        &self.remove_button
    }
}

impl PlayerSetupWidget {
    /// Create a new player setup widget with a default single player.
    pub fn new() -> Self {
        let title = gtk4::Label::builder()
            .label("Spieler einrichten")
            .css_classes(vec!["setup-title"])
            .halign(gtk4::Align::Start)
            .build();

        let rows_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(8)
            .css_classes(vec!["player-rows"])
            .hexpand(true)
            .build();

        let add_button = gtk4::Button::builder()
            .label("+ Spieler hinzufügen")
            .css_classes(vec!["add-player-button"])
            .halign(gtk4::Align::Start)
            .build();

        let start_button = gtk4::Button::builder()
            .label("Spiel starten")
            .css_classes(vec!["start-game-button", "suggested-action"])
            .halign(gtk4::Align::End)
            .hexpand(true)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["player-setup"])
            .spacing(12)
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        container.append(&title);
        container.append(&rows_box);
        container.append(&add_button);
        container.append(&start_button);

        let first_row = PlayerRow::new(0);
        rows_box.append(&first_row.box_widget);

        let rows = vec![first_row];

        Self {
            container,
            rows,
            add_button,
            start_button,
        }
    }

    /// Get the "add player" button (for connecting signals).
    pub fn add_button(&self) -> &gtk4::Button {
        &self.add_button
    }

    /// Get the "start game" button (for connecting signals).
    pub fn start_button(&self) -> &gtk4::Button {
        &self.start_button
    }

    /// Add a new player row. Returns the index of the new row,
    /// or `None` if the maximum (6) has been reached.
    pub fn add_player(&mut self) -> Option<usize> {
        let index = self.rows.len();
        if index >= 6 {
            return None;
        }
        let row = PlayerRow::new(index);
        #[allow(clippy::collapsible_if)]
        if let Some(rows_box) = self.container.first_child() {
            if let Some(rows_box) = rows_box.downcast_ref::<gtk4::Box>() {
                rows_box.append(&row.box_widget);
            }
        }
        self.rows.push(row);
        Some(index)
    }

    /// Remove the player row at the given index.
    ///
    /// Returns `true` if the row was removed, `false` if the index
    /// is out of bounds or only one player remains.
    pub fn remove_player(&mut self, index: usize) -> bool {
        if self.rows.len() <= 1 || index >= self.rows.len() {
            return false;
        }
        let row = self.rows.remove(index);
        #[allow(clippy::collapsible_if)]
        if let Some(rows_box) = self.container.first_child() {
            if let Some(rows_box) = rows_box.downcast_ref::<gtk4::Box>() {
                rows_box.remove(&row.box_widget);
            }
        }
        true
    }

    /// Get the remove button for the player at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    /// Used by the application to connect "clicked" signals.
    pub fn remove_button(&self, index: usize) -> Option<&gtk4::Button> {
        self.rows.get(index).map(|r| r.remove_button())
    }

    /// Build a validated `PlayerSetup` from the current configuration.
    pub fn build_setup(&self) -> Result<PlayerSetup> {
        let configs: Vec<PlayerConfig> = self.rows.iter().map(|r| r.to_config()).collect::<Result<Vec<_>>>()?;
        PlayerSetup::new(configs)
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for PlayerSetupWidget {
    fn default() -> Self {
        Self::new()
    }
}
