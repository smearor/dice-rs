use crate::i18n;
use crate::models::dice_set::DiceSet;
use crate::models::dice_slot::DiceSlot;
use crate::models::hold_mask::HoldMask;
use crate::models::roll_count::RollCount;
use crate::models::scorecard::Scorecard;
use crate::strategy::computer_ai::ComputerAi;
use crate::strategy::expected_value::CategoryChoice;
use gtk4::prelude::*;

/// Widget displaying strategy hints for the current player.
///
/// Shows the recommended category to enter and the recommended
/// dice to hold for re-rolling. Designed to help human players
/// learn optimal strategy.
pub struct HintPanel {
    /// The root container widget.
    container: gtk4::Box,
    /// Label showing the recommended category.
    category_label: gtk4::Label,
    /// Label showing the expected score.
    score_label: gtk4::Label,
    /// Label showing the recommended hold dice.
    hold_label: gtk4::Label,
    /// The computer AI engine for generating hints.
    ai: ComputerAi,
}

impl HintPanel {
    /// Create a new hint panel.
    pub fn new() -> Self {
        let title = gtk4::Label::builder()
            .label(&i18n::get("hint-title"))
            .css_classes(vec!["hint-title"])
            .halign(gtk4::Align::Start)
            .build();

        let category_label = gtk4::Label::builder()
            .css_classes(vec!["hint-category"])
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let score_label = gtk4::Label::builder().css_classes(vec!["hint-score"]).halign(gtk4::Align::End).build();

        let hold_label = gtk4::Label::builder()
            .css_classes(vec!["hint-holds"])
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let category_row = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).hexpand(true).build();
        category_row.append(&category_label);
        category_row.append(&score_label);

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["hint-panel"])
            .spacing(4)
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        container.append(&title);
        container.append(&category_row);
        container.append(&hold_label);

        Self {
            container,
            category_label,
            score_label,
            hold_label,
            ai: ComputerAi::new(),
        }
    }

    /// Update the hint panel with the current game state.
    pub fn update(&self, dice: &DiceSet, scorecard: &Scorecard, rolls_used: RollCount) {
        let best = self.ai.hint_best_category(dice, scorecard);
        self.show_category_hint(&best);

        if !rolls_used.is_exhausted() {
            match self.ai.hint_best_holds(dice, scorecard, rolls_used) {
                Ok(holds) => self.show_hold_hint(dice, holds),
                Err(_) => self.hold_label.set_label("—"),
            }
        } else {
            self.hold_label.set_label(&i18n::get("hint-no-more-rolls"));
        }
    }

    /// Show the category recommendation.
    fn show_category_hint(&self, choice: &CategoryChoice) {
        let text = i18n::get_str("hint-category", "category", &choice.category().to_string());
        self.category_label.set_label(&text);

        let score_text = if choice.score().get() > 0 {
            i18n::get_int("hint-score", "score", choice.score().get() as i64)
        } else {
            i18n::get("hint-score-zero")
        };
        self.score_label.set_label(&score_text);
    }

    /// Show the hold recommendation.
    fn show_hold_hint(&self, dice: &DiceSet, holds: HoldMask) {
        if holds.held_count() == 5 {
            self.hold_label.set_label(&i18n::get("hint-hold-all"));
            return;
        }

        if holds.held_count() == 0 {
            self.hold_label.set_label(&i18n::get("hint-hold-none"));
            return;
        }

        let held_values: Vec<String> = DiceSlot::all()
            .filter(|slot| holds.is_held(*slot))
            .map(|slot| dice.values()[slot.get() as usize].to_string())
            .collect();

        let text = i18n::get_str("hint-hold-some", "values", &held_values.join(", "));
        self.hold_label.set_label(&text);
    }

    /// Clear the hint panel.
    pub fn clear(&self) {
        self.category_label.set_label("");
        self.score_label.set_label("");
        self.hold_label.set_label("");
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for HintPanel {
    fn default() -> Self {
        Self::new()
    }
}
