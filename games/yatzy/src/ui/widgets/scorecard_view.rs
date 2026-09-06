use crate::i18n;
use crate::models::category::ScoreCategory;
use crate::models::category_section::CategorySection;
use crate::models::player::Player;
use crate::models::score::Score;
use crate::models::score_entry::ScoreEntry;
use crate::models::scorecard::Scorecard;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Callback type for when a category is selected for scoring.
type CategorySelectedCallback = Rc<dyn Fn(ScoreCategory)>;

/// Widget displaying all players' scorecards as a grid.
///
/// Shows all 13 categories with scores for each player, organized into
/// upper and lower sections. Empty categories in the active player's
/// column are clickable for score entry. Filled categories show their
/// score; crossed-out categories show a strike-through.
pub struct ScorecardView {
    /// The root container widget.
    container: gtk4::Box,
    /// The grid holding the scorecard rows.
    grid: gtk4::Grid,
    /// The currently displayed players (owned copies).
    players: Rc<RefCell<Vec<Player>>>,
    /// The index of the active player whose column is clickable.
    active_player: Rc<RefCell<usize>>,
    /// The last-entered (player, category) to highlight until next entry.
    last_entered: Rc<RefCell<Option<(usize, ScoreCategory)>>>,
    /// Callback invoked when a category is clicked.
    on_category_selected: Rc<RefCell<Option<CategorySelectedCallback>>>,
}

impl ScorecardView {
    /// Create a new empty scorecard view.
    pub fn new() -> Self {
        let grid = gtk4::Grid::builder()
            .css_classes(vec!["scorecard-grid"])
            .column_spacing(8)
            .row_spacing(2)
            .vexpand(true)
            .hexpand(true)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["scorecard-view"])
            .vexpand(true)
            .hexpand(true)
            .build();

        container.append(&grid);

        Self {
            container,
            grid,
            players: Rc::new(RefCell::new(Vec::new())),
            active_player: Rc::new(RefCell::new(0)),
            last_entered: Rc::new(RefCell::new(None)),
            on_category_selected: Rc::new(RefCell::new(None)),
        }
    }

    /// Set the callback invoked when a category is clicked.
    pub fn connect_category_selected<F>(&self, callback: F)
    where
        F: Fn(ScoreCategory) + 'static,
    {
        *self.on_category_selected.borrow_mut() = Some(Rc::new(callback));
    }

    /// Update the view to display all players' scorecards.
    pub fn update_players(&self, players: &[Player], active_player: usize) {
        *self.players.borrow_mut() = players.to_vec();
        *self.active_player.borrow_mut() = active_player;
        self.rebuild_grid(players, active_player);
    }

    /// Mark a cell as recently entered so it stays highlighted until the next entry.
    pub fn set_last_entered(&self, player_index: usize, category: ScoreCategory) {
        *self.last_entered.borrow_mut() = Some((player_index, category));
        let players = self.players.borrow().clone();
        let active = *self.active_player.borrow();
        self.rebuild_grid(&players, active);
    }

    /// Clear the recently-entered highlight.
    pub fn clear_last_entered(&self) {
        *self.last_entered.borrow_mut() = None;
    }

    /// Update the view to display a single player's scorecard.
    /// Kept for backward compatibility — updates the active player's column.
    pub fn update(&self, scorecard: &Scorecard) {
        let mut players = self.players.borrow_mut();
        let active = *self.active_player.borrow();
        if active < players.len() {
            players[active].scorecard_mut().clone_from(scorecard);
        }
        let players_clone = players.clone();
        drop(players);
        self.rebuild_grid(&players_clone, active);
    }

    /// Clear the scorecard view.
    pub fn clear(&self) {
        *self.players.borrow_mut() = Vec::new();
        *self.active_player.borrow_mut() = 0;
        *self.last_entered.borrow_mut() = None;
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }
    }

    /// Number of columns: 1 for category labels + 1 per player.
    fn num_columns(&self, players: &[Player]) -> i32 {
        1 + players.len() as i32
    }

    /// Rebuild the grid from all players' scorecards.
    fn rebuild_grid(&self, players: &[Player], active_player: usize) {
        // Remove all existing children
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }

        let num_cols = self.num_columns(players);
        let mut row: i32 = 0;

        // Player name header row
        row = self.add_player_header_row(row, players, active_player);

        // Upper section header
        row = self.add_section_header(row, &i18n::get("scorecard-upper-section"), num_cols);

        // Upper section categories
        for category in ScoreCategory::ALL {
            if category.section() == CategorySection::Upper {
                row = self.add_category_row(row, players, active_player, category);
            }
        }

        // Upper bonus
        row = self.add_summary_row(row, &i18n::get("scorecard-bonus"), players, Scorecard::upper_bonus, num_cols, false);

        // Upper subtotal
        row = self.add_summary_row(row, &i18n::get("scorecard-subtotal"), players, Scorecard::upper_subtotal, num_cols, false);

        // Lower section header
        row = self.add_section_header(row, &i18n::get("scorecard-lower-section"), num_cols);

        // Lower section categories
        for category in ScoreCategory::ALL {
            if category.section() == CategorySection::Lower {
                row = self.add_category_row(row, players, active_player, category);
            }
        }

        // Yatzy bonus
        row = self.add_summary_row(row, &i18n::get("scorecard-yatzy-bonus"), players, Scorecard::yatzy_bonus, num_cols, false);

        // Lower subtotal
        row = self.add_summary_row(row, &i18n::get("scorecard-subtotal"), players, Scorecard::lower_subtotal, num_cols, false);

        // Grand total
        self.add_summary_row(row, &i18n::get("scorecard-total"), players, Scorecard::grand_total, num_cols, true);
    }

    /// Add the player name header row.
    fn add_player_header_row(&self, row: i32, players: &[Player], active_player: usize) -> i32 {
        // Empty cell above category labels
        let spacer = gtk4::Label::builder().build();
        self.grid.attach(&spacer, 0, row, 1, 1);

        for (i, player) in players.iter().enumerate() {
            let name_label = gtk4::Label::builder()
                .label(player.name().as_str())
                .css_classes(vec!["scorecard-player-name"])
                .halign(gtk4::Align::Center)
                .hexpand(true)
                .build();

            if i == active_player {
                name_label.add_css_class("active");
            }

            self.grid.attach(&name_label, (i + 1) as i32, row, 1, 1);
        }

        row + 1
    }

    /// Add a section header row spanning all columns.
    fn add_section_header(&self, row: i32, title: &str, num_cols: i32) -> i32 {
        let label = gtk4::Label::builder()
            .label(title)
            .css_classes(vec!["scorecard-section-header"])
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        self.grid.attach(&label, 0, row, num_cols, 1);
        row + 1
    }

    /// Add a category row with label and per-player scores.
    fn add_category_row(&self, row: i32, players: &[Player], active_player: usize, category: ScoreCategory) -> i32 {
        // Category label — left-aligned, no hexpand so player columns start right after
        let category_label = gtk4::Label::builder()
            .label(category.to_string())
            .css_classes(vec!["scorecard-category"])
            .halign(gtk4::Align::Start)
            .build();

        self.grid.attach(&category_label, 0, row, 1, 1);

        let last_entered = self.last_entered.borrow();

        // Per-player score cells
        for (i, player) in players.iter().enumerate() {
            let scorecard = player.scorecard();
            let entry = scorecard.entry(category);
            let is_filled = entry.is_used();
            let is_active = i == active_player;
            let is_clickable = is_active && !is_filled;
            let is_recent = last_entered.is_some_and(|(p, c)| p == i && c == category);

            let cell_box = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .css_classes(vec!["scorecard-row"])
                .spacing(4)
                .halign(gtk4::Align::Center)
                .hexpand(true)
                .build();

            if is_filled {
                cell_box.add_css_class("filled");
            }
            if is_active {
                cell_box.add_css_class("active");
            }
            if is_clickable {
                cell_box.add_css_class("clickable");
            }
            if is_recent {
                cell_box.add_css_class("recently-entered");
            }

            let score_text = match entry {
                ScoreEntry::Empty => String::new(),
                ScoreEntry::Filled(score) => score.get().to_string(),
                ScoreEntry::CrossedOut => "—".to_string(),
            };

            let score_label = gtk4::Label::builder()
                .label(&score_text)
                .css_classes(vec!["scorecard-score"])
                .halign(gtk4::Align::End)
                .build();

            if matches!(entry, ScoreEntry::CrossedOut) {
                score_label.add_css_class("crossed-out");
            }

            cell_box.append(&score_label);

            // Make clickable only for the active player's unfilled categories
            if is_clickable {
                let click = gtk4::GestureClick::new();
                let category_for_click = category;
                let on_selected = self.on_category_selected.clone();
                click.connect_pressed(move |_, _, _, _| {
                    if let Some(callback) = on_selected.borrow().as_ref() {
                        callback(category_for_click);
                    }
                });
                cell_box.add_controller(click);
            }

            self.grid.attach(&cell_box, (i + 1) as i32, row, 1, 1);
        }

        row + 1
    }

    /// Add a summary row (bonus, subtotal, total) with per-player values.
    fn add_summary_row(&self, row: i32, label: &str, players: &[Player], score_fn: fn(&Scorecard) -> Score, num_cols: i32, is_grand_total: bool) -> i32 {
        let mut css_classes = vec!["scorecard-total-row"];
        if is_grand_total {
            css_classes.push("scorecard-grand-total");
        }

        let row_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(css_classes)
            .spacing(8)
            .halign(gtk4::Align::Start)
            .build();

        let label_widget = gtk4::Label::builder().label(label).halign(gtk4::Align::Start).build();
        row_box.append(&label_widget);
        self.grid.attach(&row_box, 0, row, 1, 1);

        for (i, player) in players.iter().enumerate() {
            let score = score_fn(player.scorecard());

            let score_widget = gtk4::Label::builder()
                .label(score.get().to_string())
                .css_classes(vec!["scorecard-score"])
                .halign(gtk4::Align::Center)
                .hexpand(true)
                .build();

            if i == *self.active_player.borrow() {
                score_widget.add_css_class("active");
            }

            self.grid.attach(&score_widget, (i + 1) as i32, row, 1, 1);
        }

        let _ = num_cols;
        row + 1
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for ScorecardView {
    fn default() -> Self {
        Self::new()
    }
}
