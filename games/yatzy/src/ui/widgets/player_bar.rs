use crate::models::player::Player;
use crate::models::player_index::PlayerIndex;
use gtk4::prelude::*;
use std::cell::RefCell;

/// Widget displaying the list of players with their current scores.
///
/// The active player is highlighted with a CSS class. Each player
/// card shows the player's name, color indicator, and total score.
pub struct PlayerBar {
    /// The root container widget.
    container: gtk4::Grid,
    /// Player card widgets, indexed by player index.
    player_cards: RefCell<Vec<gtk4::Box>>,
    /// Player name labels.
    name_labels: RefCell<Vec<gtk4::Label>>,
    /// Player score labels.
    score_labels: RefCell<Vec<gtk4::Label>>,
}

impl PlayerBar {
    /// Create a new empty player bar.
    pub fn new() -> Self {
        let container = gtk4::Grid::builder()
            .css_classes(vec!["player-bar"])
            .column_spacing(8)
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        Self {
            container,
            player_cards: RefCell::new(Vec::new()),
            name_labels: RefCell::new(Vec::new()),
            score_labels: RefCell::new(Vec::new()),
        }
    }

    /// Populate the player bar from a list of players.
    pub fn set_players(&self, players: &[Player]) {
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }
        self.player_cards.borrow_mut().clear();
        self.name_labels.borrow_mut().clear();
        self.score_labels.borrow_mut().clear();

        // Column 0: spacer to align with scorecard category labels
        let spacer = gtk4::Label::builder().build();
        self.container.attach(&spacer, 0, 0, 1, 1);

        for (i, player) in players.iter().enumerate() {
            let card = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Vertical)
                .css_classes(vec!["player-card", &format!("player-color-{i}")])
                .spacing(4)
                .halign(gtk4::Align::Center)
                .hexpand(true)
                .build();

            let name_label = gtk4::Label::builder()
                .label(player.name().as_str())
                .css_classes(vec!["player-name"])
                .halign(gtk4::Align::Center)
                .build();

            let score_label = gtk4::Label::builder()
                .label("0")
                .css_classes(vec!["player-score"])
                .halign(gtk4::Align::Center)
                .build();

            card.append(&name_label);
            card.append(&score_label);
            self.container.attach(&card, (i + 1) as i32, 0, 1, 1);

            self.player_cards.borrow_mut().push(card);
            self.name_labels.borrow_mut().push(name_label);
            self.score_labels.borrow_mut().push(score_label);
        }
    }

    /// Highlight the active player.
    pub fn set_active_player(&self, index: PlayerIndex) {
        for (i, card) in self.player_cards.borrow().iter().enumerate() {
            if i == index.get() {
                card.add_css_class("active");
            } else {
                card.remove_css_class("active");
            }
        }
    }

    /// Update the displayed score for a player.
    pub fn update_score(&self, index: PlayerIndex, score: u32) {
        if let Some(label) = self.score_labels.borrow().get(index.get()) {
            label.set_label(&score.to_string());
        }
    }

    /// Update all scores from a list of players.
    pub fn update_all_scores(&self, players: &[Player]) {
        for (i, player) in players.iter().enumerate() {
            if let Some(label) = self.score_labels.borrow().get(i) {
                let total = player.scorecard().grand_total().get();
                label.set_label(&total.to_string());
            }
        }
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for PlayerBar {
    fn default() -> Self {
        Self::new()
    }
}
