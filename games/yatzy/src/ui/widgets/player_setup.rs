use crate::error::Result;
use crate::i18n;
use crate::models::game_settings::GameSettings;
use crate::models::game_settings::PlayerSettingsEntry;
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
    /// The box holding player rows.
    rows_box: gtk4::Box,
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
    /// The color picker button.
    color_button: gtk4::ColorButton,
    /// The type dropdown (Human/Computer).
    type_dropdown: gtk4::DropDown,
    /// The remove button.
    remove_button: gtk4::Button,
}

impl PlayerRow {
    /// Create a new player row with default values.
    fn new(index: usize) -> Self {
        let name_entry = gtk4::Entry::builder()
            .text(i18n::get_int("player-default-name", "index", (index + 1) as i64))
            .css_classes(vec!["player-name-entry"])
            .hexpand(true)
            .build();

        let default_color = PlayerColor::DEFAULTS[index % PlayerColor::DEFAULTS.len()];
        let rgba = gtk4::gdk::RGBA::new(
            default_color.led_color().r as f32 / 255.0,
            default_color.led_color().g as f32 / 255.0,
            default_color.led_color().b as f32 / 255.0,
            1.0,
        );
        let color_button = gtk4::ColorButton::builder()
            .rgba(&rgba)
            .css_classes(vec!["player-color-button"])
            .tooltip_text(&i18n::get("player-color-tooltip"))
            .build();

        let type_model = gtk4::StringList::new(&[&i18n::get("player-type-human"), &i18n::get("player-type-computer")]);
        let type_dropdown = gtk4::DropDown::builder()
            .model(&type_model)
            .selected(0)
            .css_classes(vec!["player-type-dropdown"])
            .build();

        let remove_button = gtk4::Button::builder()
            .label("✕")
            .css_classes(vec!["player-remove-button"])
            .tooltip_text(&i18n::get("player-remove-tooltip"))
            .build();

        let box_widget = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .css_classes(vec!["player-row"])
            .hexpand(true)
            .build();

        box_widget.append(&name_entry);
        box_widget.append(&color_button);
        box_widget.append(&type_dropdown);
        box_widget.append(&remove_button);

        Self {
            box_widget,
            name_entry,
            color_button,
            type_dropdown,
            remove_button,
        }
    }

    /// Get the configured player name from the entry field.
    fn player_name(&self) -> Result<PlayerName> {
        let text = self.name_entry.text().to_string();
        PlayerName::new(text)
    }

    /// Get the configured player color from the color button.
    fn player_color(&self) -> PlayerColor {
        let rgba = self.color_button.rgba();
        PlayerColor::new(dice_rs::LedColor::new(
            (rgba.red() * 255.0).round() as u8,
            (rgba.green() * 255.0).round() as u8,
            (rgba.blue() * 255.0).round() as u8,
        ))
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
            .label(&i18n::get("setup-player-title"))
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
            .label(&i18n::get("player-add"))
            .css_classes(vec!["add-player-button"])
            .halign(gtk4::Align::Start)
            .build();

        let start_button = gtk4::Button::builder()
            .label(&i18n::get("player-start"))
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
            rows_box,
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

    /// Enable or disable the "start game" button.
    pub fn set_start_button_sensitive(&self, sensitive: bool) {
        self.start_button.set_sensitive(sensitive);
    }

    /// Add a new player row. Returns the index of the new row,
    /// or `None` if the maximum (6) has been reached.
    pub fn add_player(&mut self) -> Option<usize> {
        let index = self.rows.len();
        if index >= 6 {
            return None;
        }
        let row = PlayerRow::new(index);
        self.rows_box.append(&row.box_widget);
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
        self.rows_box.remove(&row.box_widget);
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

    /// Load player settings into the widget, replacing all current rows.
    ///
    /// Removes all existing rows and creates new ones matching the
    /// provided settings. If settings have no players, a single default
    /// row is kept.
    pub fn load_settings(&mut self, settings: &GameSettings) {
        // Remove all existing rows
        while let Some(row) = self.rows.pop() {
            self.rows_box.remove(&row.box_widget);
        }

        let players = settings.players();
        if players.is_empty() {
            let row = PlayerRow::new(0);
            self.rows_box.append(&row.box_widget);
            self.rows.push(row);
            return;
        }

        for (index, entry) in players.iter().enumerate() {
            let row = PlayerRow::new(index);
            row.name_entry.set_text(entry.name().as_str());
            let rgba = gtk4::gdk::RGBA::new(
                entry.color().led_color().r as f32 / 255.0,
                entry.color().led_color().g as f32 / 255.0,
                entry.color().led_color().b as f32 / 255.0,
                1.0,
            );
            row.color_button.set_rgba(&rgba);
            let type_index = if entry.player_type() == PlayerType::Human { 0 } else { 1 };
            row.type_dropdown.set_selected(type_index);
            self.rows_box.append(&row.box_widget);
            self.rows.push(row);
        }
    }

    /// Collect the current configuration as `GameSettings`.
    ///
    /// Returns the current player rows as a `GameSettings` object
    /// suitable for persisting to disk. Uses `MultiPlayer` mode if
    /// there are 2+ players, `SinglePlayer` otherwise.
    pub fn collect_settings(&self) -> Result<GameSettings> {
        let entries: Vec<PlayerSettingsEntry> = self
            .rows
            .iter()
            .map(|r| Ok(PlayerSettingsEntry::new(r.player_name()?, r.player_color(), r.player_type())))
            .collect::<Result<Vec<_>>>()?;
        let mode = if entries.len() > 1 {
            crate::models::game_mode::GameMode::MultiPlayer
        } else {
            crate::models::game_mode::GameMode::SinglePlayer
        };
        Ok(GameSettings::new(mode, entries))
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
