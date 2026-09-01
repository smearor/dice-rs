use crate::models::category::ScoreCategory;
use crate::models::category_section::CategorySection;
use crate::models::score::Score;
use crate::models::score_entry::ScoreEntry;
use crate::models::scorecard::Scorecard;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Callback type for when a category is selected for scoring.
type CategorySelectedCallback = Rc<dyn Fn(ScoreCategory)>;

/// Widget displaying a player's scorecard as a grid.
///
/// Shows all 13 categories with their current scores, organized into
/// upper and lower sections. Empty categories are clickable for
/// score entry. Filled categories show their score; crossed-out
/// categories show a strike-through.
pub struct ScorecardView {
    /// The root container widget.
    container: gtk4::Box,
    /// The grid holding the scorecard rows.
    grid: gtk4::Grid,
    /// The currently displayed scorecard (owned copy).
    scorecard: Rc<RefCell<Option<Scorecard>>>,
    /// Callback invoked when a category is clicked.
    on_category_selected: Rc<RefCell<Option<CategorySelectedCallback>>>,
}

impl ScorecardView {
    /// Create a new empty scorecard view.
    pub fn new() -> Self {
        let grid = gtk4::Grid::builder()
            .css_classes(vec!["scorecard-grid"])
            .column_spacing(4)
            .row_spacing(2)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["scorecard-view"])
            .build();

        container.append(&grid);

        Self {
            container,
            grid,
            scorecard: Rc::new(RefCell::new(None)),
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

    /// Update the view to display a scorecard.
    pub fn update(&self, scorecard: &Scorecard) {
        *self.scorecard.borrow_mut() = Some(scorecard.clone());
        self.rebuild_grid(scorecard);
    }

    /// Clear the scorecard view.
    pub fn clear(&self) {
        *self.scorecard.borrow_mut() = None;
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }
    }

    /// Rebuild the grid from a scorecard.
    fn rebuild_grid(&self, scorecard: &Scorecard) {
        // Remove all existing children
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }

        let mut row: i32 = 0;

        // Upper section header
        row = self.add_section_header(row, "Obere Hälfte");

        // Upper section categories
        for category in ScoreCategory::ALL {
            if category.section() == CategorySection::Upper {
                row = self.add_category_row(row, scorecard, category);
            }
        }

        // Upper bonus
        row = self.add_bonus_row(row, "Bonus", scorecard.upper_bonus());

        // Upper subtotal
        row = self.add_subtotal_row(row, "Zwischensumme", scorecard.upper_subtotal());

        // Lower section header
        row = self.add_section_header(row, "Untere Hälfte");

        // Lower section categories
        for category in ScoreCategory::ALL {
            if category.section() == CategorySection::Lower {
                row = self.add_category_row(row, scorecard, category);
            }
        }

        // Yahtzee bonus
        row = self.add_bonus_row(row, "Kniffel Bonus", scorecard.yahtzee_bonus());

        // Lower subtotal
        row = self.add_subtotal_row(row, "Zwischensumme", scorecard.lower_subtotal());

        // Grand total
        self.add_total_row(row, "Gesamt", scorecard.grand_total());
    }

    /// Add a section header row.
    fn add_section_header(&self, row: i32, title: &str) -> i32 {
        let label = gtk4::Label::builder()
            .label(title)
            .css_classes(vec!["scorecard-section-header"])
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        self.grid.attach(&label, 0, row, 2, 1);
        row + 1
    }

    /// Add a category row with label and score.
    fn add_category_row(&self, row: i32, scorecard: &Scorecard, category: ScoreCategory) -> i32 {
        let entry = scorecard.entry(category);
        let is_filled = entry.is_used();

        let row_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["scorecard-row"])
            .spacing(8)
            .hexpand(true)
            .build();

        if is_filled {
            row_box.add_css_class("filled");
        }

        let category_label = gtk4::Label::builder()
            .label(category.to_string())
            .css_classes(vec!["scorecard-category"])
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let score_text = match entry {
            ScoreEntry::Empty => String::new(),
            ScoreEntry::Filled(score) => score.get().to_string(),
            ScoreEntry::CrossedOut => "—".to_string(),
        };

        if matches!(entry, ScoreEntry::CrossedOut) {
            category_label.add_css_class("crossed-out");
        }

        let score_label = gtk4::Label::builder()
            .label(&score_text)
            .css_classes(vec!["scorecard-score"])
            .halign(gtk4::Align::End)
            .build();

        row_box.append(&category_label);
        row_box.append(&score_label);

        // Make clickable if not filled
        if !is_filled {
            let click = gtk4::GestureClick::new();
            let category_for_click = category;
            let on_selected = self.on_category_selected.clone();
            click.connect_released(move |_, _, _, _| {
                if let Some(callback) = on_selected.borrow().as_ref() {
                    callback(category_for_click);
                }
            });
            row_box.add_controller(click);
        }

        self.grid.attach(&row_box, 0, row, 2, 1);
        row + 1
    }

    /// Add a bonus row.
    fn add_bonus_row(&self, row: i32, label: &str, score: Score) -> i32 {
        let row_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["scorecard-bonus-row"])
            .spacing(8)
            .hexpand(true)
            .build();

        let label_widget = gtk4::Label::builder().label(label).halign(gtk4::Align::Start).hexpand(true).build();

        let score_widget = gtk4::Label::builder()
            .label(score.get().to_string())
            .css_classes(vec!["scorecard-score"])
            .halign(gtk4::Align::End)
            .build();

        row_box.append(&label_widget);
        row_box.append(&score_widget);
        self.grid.attach(&row_box, 0, row, 2, 1);
        row + 1
    }

    /// Add a subtotal row.
    fn add_subtotal_row(&self, row: i32, label: &str, score: Score) -> i32 {
        let row_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["scorecard-total-row"])
            .spacing(8)
            .hexpand(true)
            .build();

        let label_widget = gtk4::Label::builder().label(label).halign(gtk4::Align::Start).hexpand(true).build();

        let score_widget = gtk4::Label::builder()
            .label(score.get().to_string())
            .css_classes(vec!["scorecard-score"])
            .halign(gtk4::Align::End)
            .build();

        row_box.append(&label_widget);
        row_box.append(&score_widget);
        self.grid.attach(&row_box, 0, row, 2, 1);
        row + 1
    }

    /// Add the grand total row.
    fn add_total_row(&self, row: i32, label: &str, score: Score) {
        let row_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .css_classes(vec!["scorecard-total-row"])
            .spacing(8)
            .hexpand(true)
            .build();

        let label_widget = gtk4::Label::builder().label(label).halign(gtk4::Align::Start).hexpand(true).build();

        let score_widget = gtk4::Label::builder()
            .label(score.get().to_string())
            .css_classes(vec!["scorecard-score"])
            .halign(gtk4::Align::End)
            .build();

        row_box.append(&label_widget);
        row_box.append(&score_widget);
        self.grid.attach(&row_box, 0, row, 2, 1);
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
