use std::cell::RefCell;
use std::rc::Rc;

use dice_rs::model::dice::DiceColor;
use dice_rs::model::face::FaceValue;
use dice_rs::model::stability_descriptor::StabilityDescriptor;
use gtk4::prelude::*;

use crate::platform::widget_container::WidgetContainer;
use crate::styling::stability::StabilityDescriptorStyle;

/// Number of history entries to display.
const HISTORY_SIZE: usize = 10;

/// A single history entry with its stability descriptor.
struct HistoryEntry {
    face: FaceValue,
    stability: StabilityDescriptor,
}

/// Displays the history of the last stable rolls.
///
/// Cloneable - all clones share the same underlying GTK widgets.
#[derive(Clone)]
pub struct RollHistory {
    container: gtk4::Box,
    labels: Vec<gtk4::Label>,
    history: Rc<RefCell<Vec<HistoryEntry>>>,
    dice_color: Rc<RefCell<DiceColor>>,
}

impl RollHistory {
    /// Create a new roll history widget.
    pub fn new() -> Self {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(4)
            .css_classes(vec!["roll-history"])
            .build();

        let title = gtk4::Label::builder().label("History:").css_classes(vec!["dim-label"]).build();
        container.append(&title);

        let mut labels = Vec::with_capacity(HISTORY_SIZE);
        for _ in 0..HISTORY_SIZE {
            let label = gtk4::Label::builder().label("-").css_classes(vec!["roll-history-entry"]).build();
            container.append(&label);
            labels.push(label);
        }

        Self {
            container,
            labels,
            history: Rc::new(RefCell::new(Vec::new())),
            dice_color: Rc::new(RefCell::new(DiceColor::Black)),
        }
    }

    /// Set the dice physical color for coloring stable entries.
    pub fn set_dice_color(&self, color: DiceColor) {
        *self.dice_color.borrow_mut() = color;
        self.refresh_display();
    }

    /// Add a stable roll to the history and update the display.
    pub fn add_roll(&self, face: FaceValue, stability: StabilityDescriptor) {
        let mut history = self.history.borrow_mut();
        history.push(HistoryEntry { face, stability });
        if history.len() > HISTORY_SIZE {
            history.remove(0);
        }
        drop(history);

        self.refresh_display();
    }

    fn refresh_display(&self) {
        let history = self.history.borrow();
        let dice_color = *self.dice_color.borrow();
        let len = history.len();

        for (i, label) in self.labels.iter().enumerate() {
            if i < len {
                if let Some(entry) = history.get(len - 1 - i) {
                    label.set_label(&entry.face.to_string());
                    StabilityDescriptorStyle::from(entry.stability).apply_to(label, dice_color);
                }
            } else {
                label.set_label("-");
                StabilityDescriptorStyle::from(StabilityDescriptor::Rolling).apply_to(label, dice_color);
            }
        }
    }
}

impl WidgetContainer for RollHistory {
    fn widget(&self) -> &gtk4::Widget {
        self.container.as_ref()
    }
}

impl Default for RollHistory {
    fn default() -> Self {
        Self::new()
    }
}
