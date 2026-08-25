use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::glib;

/// Duration in milliseconds for which the tap indicator stays visible.
const FLASH_DURATION_MS: u64 = 1000;

/// Displays tap and double-tap events with a transient flash label.
///
/// Cloneable - all clones share the same underlying GTK widget.
#[derive(Clone)]
pub struct TapIndicator {
    label: gtk4::Label,
    flash_timeout_id: Rc<RefCell<Option<glib::SourceId>>>,
}

impl TapIndicator {
    /// Create a new tap indicator widget, initially hidden.
    pub fn new() -> Self {
        let label = gtk4::Label::builder()
            .label("")
            .css_classes(vec!["tap-indicator"])
            .visible(false)
            .build();
        Self {
            label,
            flash_timeout_id: Rc::new(RefCell::new(None)),
        }
    }

    /// Flash a single tap notification.
    pub fn flash_tap(&self) {
        self.show_flash("Tap", "tap-single");
    }

    /// Flash a double tap notification.
    pub fn flash_double_tap(&self) {
        self.show_flash("Double Tap", "tap-double");
    }

    fn show_flash(&self, text: &str, css_class: &str) {
        if let Some(id) = self.flash_timeout_id.borrow_mut().take() {
            id.remove();
        }

        self.label.remove_css_class("tap-single");
        self.label.remove_css_class("tap-double");
        self.label.set_text(text);
        self.label.add_css_class(css_class);
        self.label.set_visible(true);

        let label = self.label.clone();
        let timeout_id = glib::timeout_add_local(
            std::time::Duration::from_millis(FLASH_DURATION_MS),
            move || {
                label.set_visible(false);
                label.remove_css_class("tap-single");
                label.remove_css_class("tap-double");
                glib::ControlFlow::Break
            },
        );
        *self.flash_timeout_id.borrow_mut() = Some(timeout_id);
    }

    /// Returns the label widget for packing.
    pub fn widget(&self) -> &gtk4::Label {
        &self.label
    }
}

impl Default for TapIndicator {
    fn default() -> Self {
        Self::new()
    }
}
