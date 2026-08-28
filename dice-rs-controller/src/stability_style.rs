use dice_rs::model::dice::DiceColor;
use dice_rs::model::stability_descriptor::StabilityDescriptor;
use gtk4::prelude::*;

use crate::dice_style::DiceColorStyle;

/// Map a StabilityDescriptor to a CSS class name.
pub struct StabilityDescriptorStyle(StabilityDescriptor);

impl StabilityDescriptorStyle {
    /// Map a StabilityDescriptor to a CSS class name.
    pub fn css_class(&self) -> &'static str {
        match &self.0 {
            StabilityDescriptor::Rolling => "stability-rolling",
            StabilityDescriptor::Stable => "stability-stable",
            StabilityDescriptor::TiltStable => "stability-tilt",
            StabilityDescriptor::FakeStable => "stability-fake",
            StabilityDescriptor::MoveStable => "stability-move",
        }
    }

    pub fn all_css_classes() -> [&'static str; 5] {
        ["stability-rolling", "stability-stable", "stability-move", "stability-tilt", "stability-fake"]
    }

    /// Apply stability-based CSS classes to a widget.
    /// Removes previous stability classes first.
    pub fn apply_to(&self, widget: &impl WidgetExt, dice_color: DiceColor) {
        for class in Self::all_css_classes() {
            widget.remove_css_class(class);
        }
        for class in DiceColorStyle::all_bg_css_classes() {
            widget.remove_css_class(class);
        }

        let dice_color_style = DiceColorStyle::from(dice_color);

        match self.0 {
            StabilityDescriptor::Stable => {
                widget.add_css_class(self.css_class());
                widget.add_css_class(dice_color_style.bg_css_class());
            }
            StabilityDescriptor::MoveStable => {
                widget.add_css_class(self.css_class());
            }
            StabilityDescriptor::TiltStable => {
                widget.add_css_class(self.css_class());
            }
            StabilityDescriptor::FakeStable => {
                widget.add_css_class(self.css_class());
            }
            StabilityDescriptor::Rolling => {},
        }
    }
}

impl From<StabilityDescriptor> for StabilityDescriptorStyle {
    fn from(value: StabilityDescriptor) -> Self {
        Self(value)
    }
}
