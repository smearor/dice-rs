use std::cell::RefCell;
use std::rc::Rc;

use dice_rs::model::dice::DiceColor;
use dice_rs::model::face::FaceValue;
use dice_rs::model::stability_descriptor::StabilityDescriptor;
use gtk4::prelude::*;

/// Number of history entries to display.
const HISTORY_SIZE: usize = 10;

/// Map a DiceColor to a CSS background class name.
fn dice_color_bg_class(color: DiceColor) -> &'static str {
    match color {
        DiceColor::Black => "dice-bg-black",
        DiceColor::Red => "dice-bg-red",
        DiceColor::Green => "dice-bg-green",
        DiceColor::Blue => "dice-bg-blue",
        DiceColor::Yellow => "dice-bg-yellow",
        DiceColor::Orange => "dice-bg-orange",
    }
}

/// Apply stability-based CSS classes to a widget.
/// Removes previous stability classes first.
fn apply_stability_classes(widget: &impl gtk4::prelude::WidgetExt, stability: StabilityDescriptor, dice_color: DiceColor) {
    let classes = [
        "stability-stable",
        "stability-move",
        "stability-tilt",
        "stability-fake",
        "dice-bg-black",
        "dice-bg-red",
        "dice-bg-green",
        "dice-bg-blue",
        "dice-bg-yellow",
        "dice-bg-orange",
    ];
    for class in classes {
        widget.remove_css_class(class);
    }

    match stability {
        StabilityDescriptor::Stable => {
            widget.add_css_class("stability-stable");
            widget.add_css_class(dice_color_bg_class(dice_color));
        }
        StabilityDescriptor::MoveStable => {
            widget.add_css_class("stability-move");
        }
        StabilityDescriptor::TiltStable => {
            widget.add_css_class("stability-tilt");
        }
        StabilityDescriptor::FakeStable => {
            widget.add_css_class("stability-fake");
        }
        StabilityDescriptor::Rolling => {}
    }
}

/// Displays the current face value of a dice with visual feedback for stability state.
///
/// Cloneable - all clones share the same underlying GTK widgets.
#[derive(Clone)]
pub struct FaceDisplay {
    label: gtk4::Label,
    stability_label: gtk4::Label,
    css_classes: RefCell<Vec<String>>,
    dice_color: Rc<RefCell<DiceColor>>,
}

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
            let label = gtk4::Label::builder()
                .label("-")
                .css_classes(vec!["roll-history-entry"])
                .build();
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
                    apply_stability_classes(label, entry.stability, dice_color);
                }
            } else {
                label.set_label("-");
                apply_stability_classes(label, StabilityDescriptor::Rolling, dice_color);
            }
        }
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }
}

impl Default for RollHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl FaceDisplay {
    /// Create a new face display widget.
    pub fn new() -> Self {
        let label = gtk4::Label::builder().label("?").css_classes(vec!["face-display", "face-unknown"]).build();
        let stability_label = gtk4::Label::builder().label("").css_classes(vec!["stability-label"]).build();
        Self {
            label,
            stability_label,
            css_classes: RefCell::new(Vec::new()),
            dice_color: Rc::new(RefCell::new(DiceColor::Black)),
        }
    }

    /// Set the dice physical color for coloring the stability label.
    pub fn set_dice_color(&self, color: DiceColor) {
        *self.dice_color.borrow_mut() = color;
    }

    /// Set the face value and update styling.
    pub fn set_face(&self, face: FaceValue) {
        self.label.set_label(&face.to_string());
        self.set_css_class("face-stable");
    }

    /// Show rolling state.
    pub fn set_rolling(&self) {
        self.label.set_label("...");
        self.set_css_class("face-rolling");
        self.stability_label.set_text("rolling");
    }

    /// Show disconnected state.
    pub fn set_disconnected(&self) {
        self.label.set_label("-");
        self.set_css_class("face-disconnected");
        self.stability_label.set_text("disconnected");
    }

    /// Set the stability descriptor label and apply colored styling.
    pub fn set_stability(&self, stability: StabilityDescriptor) {
        let text = match stability {
            StabilityDescriptor::Rolling => "rolling",
            StabilityDescriptor::Stable => "stable",
            StabilityDescriptor::TiltStable => "tilt",
            StabilityDescriptor::FakeStable => "fake",
            StabilityDescriptor::MoveStable => "move",
        };
        self.stability_label.set_text(text);
        let dice_color = *self.dice_color.borrow();
        apply_stability_classes(&self.stability_label, stability, dice_color);
    }

    /// Mark the face as tilted.
    pub fn set_tilted(&self, tilted: bool) {
        if tilted {
            self.set_css_class("face-tilted");
        }
    }

    /// Mark the face as fake stable.
    pub fn set_fake(&self, fake: bool) {
        if fake {
            self.set_css_class("face-fake");
        }
    }

    /// Returns the face value label widget.
    pub fn widget(&self) -> &gtk4::Label {
        &self.label
    }

    /// Returns the stability descriptor label widget.
    pub fn stability_label(&self) -> &gtk4::Label {
        &self.stability_label
    }

    fn set_css_class(&self, class: &str) {
        let mut classes = self.css_classes.borrow_mut();
        for old in classes.drain(..) {
            self.label.remove_css_class(&old);
        }
        self.label.add_css_class(class);
        classes.push(class.to_string());
    }
}

impl Default for FaceDisplay {
    fn default() -> Self {
        Self::new()
    }
}
