use dice_rs::DiceColor;

/// Map a DiceColor to a CSS class name for the border.
pub struct DiceColorStyle(DiceColor);

impl DiceColorStyle {
    /// Map a DiceColor to a CSS border class name.
    pub fn border_css_class(&self) -> &'static str {
        match &self.0 {
            DiceColor::Black => "dice-border-black",
            DiceColor::Red => "dice-border-red",
            DiceColor::Green => "dice-border-green",
            DiceColor::Blue => "dice-border-blue",
            DiceColor::Yellow => "dice-border-yellow",
            DiceColor::Orange => "dice-border-orange",
        }
    }

    /// Map a DiceColor to a CSS background class name.
    pub fn bg_css_class(&self) -> &'static str {
        match &self.0 {
            DiceColor::Black => "dice-bg-black",
            DiceColor::Red => "dice-bg-red",
            DiceColor::Green => "dice-bg-green",
            DiceColor::Blue => "dice-bg-blue",
            DiceColor::Yellow => "dice-bg-yellow",
            DiceColor::Orange => "dice-bg-orange",
        }
    }

    pub fn all_bg_css_classes() -> [&'static str; 6] {
        [
            "dice-bg-black",
            "dice-bg-red",
            "dice-bg-green",
            "dice-bg-blue",
            "dice-bg-yellow",
            "dice-bg-orange",
        ]
    }
}

impl From<DiceColor> for DiceColorStyle {
    fn from(value: DiceColor) -> Self {
        Self(value)
    }
}
