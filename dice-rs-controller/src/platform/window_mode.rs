/// Estimated titlebar height in pixels for window size calculations.
const TITLEBAR_HEIGHT: i32 = 48;

/// Window height in compact mode: margin_top(6) + widget(80) + margin_bottom(6) + content margin(24) + titlebar.
const COMPACT_HEIGHT: i32 = 6 + 80 + 6 + 24 + TITLEBAR_HEIGHT;

/// Window display mode controlling layout and size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowMode {
    Normal,
    Compact,
}

impl WindowMode {
    /// Returns `true` if this is compact mode.
    pub fn is_compact(self) -> bool {
        self == Self::Compact
    }

    /// Returns the GTK orientation for the dice list in this mode.
    pub fn orientation(self) -> gtk4::Orientation {
        if self.is_compact() {
            gtk4::Orientation::Horizontal
        } else {
            gtk4::Orientation::Vertical
        }
    }

    /// Returns the target window height for this mode given the dice count.
    pub fn window_height(self, dice_count: usize) -> i32 {
        match self {
            Self::Compact => COMPACT_HEIGHT,
            Self::Normal => {
                if dice_count == 0 {
                    1000
                } else {
                    24 + 280 * dice_count as i32 + 12 * dice_count.saturating_sub(1) as i32 + TITLEBAR_HEIGHT
                }
            }
        }
    }

    /// Derives the window mode from a window height value.
    pub fn from_height(height: i32) -> Self {
        if height <= COMPACT_HEIGHT { Self::Compact } else { Self::Normal }
    }

    /// Returns the minimum window height (compact mode height).
    pub fn min_height() -> i32 {
        COMPACT_HEIGHT
    }
}

impl From<bool> for WindowMode {
    fn from(compact: bool) -> Self {
        if compact { Self::Compact } else { Self::Normal }
    }
}
