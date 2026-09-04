use crate::i18n;
use crate::models::standing::StandingEntry;
use gtk4::prelude::*;

/// Widget displaying the final game results.
///
/// Shows the winner prominently, followed by a ranked list of all
/// players with their final scores. Includes a "New Game" button
/// to restart the game.
pub struct GameEndScreen {
    /// The root container widget.
    container: gtk4::Box,
    /// Label showing the winner's name.
    winner_label: gtk4::Label,
    /// The list box for player standings.
    standings_list: gtk4::ListBox,
    /// The "new game" button.
    new_game_button: gtk4::Button,
}

impl GameEndScreen {
    /// Create a new game end screen.
    pub fn new() -> Self {
        let title = gtk4::Label::builder()
            .label(&i18n::get("game-end-title"))
            .css_classes(vec!["game-end-title"])
            .halign(gtk4::Align::Center)
            .build();

        let winner_label = gtk4::Label::builder().css_classes(vec!["game-end-winner"]).halign(gtk4::Align::Center).build();

        let standings_list = gtk4::ListBox::builder()
            .css_classes(vec!["game-end-standings"])
            .halign(gtk4::Align::Center)
            .hexpand(true)
            .build();

        let new_game_button = gtk4::Button::builder()
            .label(&i18n::get("game-end-new-game"))
            .css_classes(vec!["game-end-new-game-button", "suggested-action"])
            .halign(gtk4::Align::Center)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["game-end-screen"])
            .spacing(16)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        container.append(&title);
        container.append(&winner_label);
        container.append(&standings_list);
        container.append(&new_game_button);

        Self {
            container,
            winner_label,
            standings_list,
            new_game_button,
        }
    }

    /// Update the end screen with the final standings.
    pub fn update(&self, standings: &[StandingEntry]) {
        // Clear existing rows
        while let Some(row) = self.standings_list.first_child() {
            self.standings_list.remove(&row);
        }

        // Display winner
        if let Some(first) = standings.first() {
            let winner_text = i18n::get_str_str("game-end-winner", "rank", &first.rank().to_string(), "name", first.player().name().as_str());
            self.winner_label.set_label(&winner_text);
        }

        // Add each player's result
        for entry in standings {
            let row = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(12)
                .css_classes(vec!["standing-row"])
                .halign(gtk4::Align::Center)
                .build();

            let rank_label = gtk4::Label::builder()
                .label(entry.rank().to_string())
                .css_classes(vec!["standing-rank"])
                .build();

            let name_label = gtk4::Label::builder()
                .label(entry.player().name().as_str())
                .css_classes(vec!["standing-name"])
                .hexpand(true)
                .build();

            let score_label = gtk4::Label::builder()
                .label(i18n::get_int("game-end-score", "score", entry.final_score().get() as i64))
                .css_classes(vec!["standing-score"])
                .build();

            row.append(&rank_label);
            row.append(&name_label);
            row.append(&score_label);

            let list_row = gtk4::ListBoxRow::builder().child(&row).build();

            self.standings_list.append(&list_row);
        }
    }

    /// Get the "new game" button (for connecting signals).
    pub fn new_game_button(&self) -> &gtk4::Button {
        &self.new_game_button
    }

    /// Clear the end screen.
    pub fn clear(&self) {
        self.winner_label.set_label("");
        while let Some(row) = self.standings_list.first_child() {
            self.standings_list.remove(&row);
        }
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for GameEndScreen {
    fn default() -> Self {
        Self::new()
    }
}
