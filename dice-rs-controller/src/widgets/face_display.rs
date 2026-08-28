use std::cell::RefCell;
use std::rc::Rc;

use crate::platform::widget_container::WidgetContainer;
use crate::styling::stability::StabilityDescriptorStyle;
use dice_rs::model::dice::DiceColor;
use dice_rs::model::face::FaceValue;
use dice_rs::model::stability_descriptor::StabilityDescriptor;
use gtk4::prelude::*;

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
        self.stability_label.set_text(stability.label());
        let dice_color = *self.dice_color.borrow();
        StabilityDescriptorStyle::from(stability).apply_to(&self.stability_label, dice_color);
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

impl WidgetContainer for FaceDisplay {
    fn widget(&self) -> &gtk4::Widget {
        self.label.as_ref()
    }
}

impl Default for FaceDisplay {
    fn default() -> Self {
        Self::new()
    }
}
