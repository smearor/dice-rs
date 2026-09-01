use crate::models::category_section::CategorySection;
use crate::models::scorecard::Scorecard;
use gtk4::prelude::*;

/// Widget displaying the upper section bonus progress.
///
/// Shows the current upper section subtotal, the progress toward
/// the 63-point bonus threshold, and whether the bonus has been achieved.
pub struct BonusTracker {
    /// The root container widget.
    container: gtk4::Box,
    /// Label showing the current subtotal.
    subtotal_label: gtk4::Label,
    /// The progress bar showing progress toward 63 points.
    progress_bar: gtk4::ProgressBar,
    /// Label showing the bonus status.
    bonus_label: gtk4::Label,
}

impl BonusTracker {
    /// Create a new bonus tracker.
    pub fn new() -> Self {
        let title = gtk4::Label::builder()
            .label("Bonus-Fortschritt")
            .css_classes(vec!["bonus-tracker-title"])
            .halign(gtk4::Align::Start)
            .build();

        let subtotal_label = gtk4::Label::builder()
            .css_classes(vec!["bonus-subtotal"])
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let progress_bar = gtk4::ProgressBar::builder()
            .css_classes(vec!["bonus-progress"])
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        let bonus_label = gtk4::Label::builder()
            .css_classes(vec!["bonus-status"])
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["bonus-tracker"])
            .spacing(4)
            .halign(gtk4::Align::Fill)
            .hexpand(true)
            .build();

        container.append(&title);
        container.append(&subtotal_label);
        container.append(&progress_bar);
        container.append(&bonus_label);

        Self {
            container,
            subtotal_label,
            progress_bar,
            bonus_label,
        }
    }

    /// Update the bonus tracker with the current scorecard.
    pub fn update(&self, scorecard: &Scorecard) {
        let subtotal = scorecard.upper_subtotal();
        let remaining = scorecard.upper_bonus_remaining();
        let bonus = scorecard.upper_bonus();

        let subtotal_text = format!("Obere Hälfte: {}/{}", subtotal.get(), CategorySection::UPPER_BONUS_THRESHOLD);
        self.subtotal_label.set_label(&subtotal_text);

        let fraction = if subtotal.get() >= CategorySection::UPPER_BONUS_THRESHOLD {
            1.0
        } else {
            subtotal.get() as f64 / CategorySection::UPPER_BONUS_THRESHOLD as f64
        };
        self.progress_bar.set_fraction(fraction);

        let bonus_text = if bonus.get() > 0 {
            format!("✓ Bonus erreicht: +{} Punkte", bonus.get())
        } else if remaining.get() == 0 {
            "Bonus erreicht!".to_string()
        } else {
            format!("Noch {} Punkte bis zum Bonus", remaining.get())
        };
        self.bonus_label.set_label(&bonus_text);

        if bonus.get() > 0 {
            self.bonus_label.add_css_class("bonus-achieved");
        } else {
            self.bonus_label.remove_css_class("bonus-achieved");
        }
    }

    /// Clear the bonus tracker.
    pub fn clear(&self) {
        self.subtotal_label.set_label("");
        self.progress_bar.set_fraction(0.0);
        self.bonus_label.set_label("");
        self.bonus_label.remove_css_class("bonus-achieved");
    }

    /// Get the root widget.
    pub fn widget(&self) -> &gtk4::Widget {
        self.container.upcast_ref()
    }
}

impl Default for BonusTracker {
    fn default() -> Self {
        Self::new()
    }
}
