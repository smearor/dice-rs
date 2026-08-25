use std::cell::RefCell;
use std::rc::Rc;

use dice_rs::service::dice::Dice;
use glib::clone;
use gtk4::prelude::*;
use tracing::debug;

/// Tap and double tap interrupt controls for a connected dice.
pub struct TapControls {
    container: gtk4::Box,
    dice: Rc<RefCell<Option<Dice>>>,
}

impl TapControls {
    /// Create a new tap controls panel.
    pub fn new() -> Self {
        let tap_label = gtk4::Label::builder().label("Tap").build();
        let tap_switch = gtk4::Switch::builder().tooltip_text("Enable / disable tap notifications").build();

        let double_tap_label = gtk4::Label::builder().label("Double Tap").build();
        let double_tap_switch = gtk4::Switch::builder().tooltip_text("Enable / disable double tap notifications").build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .css_classes(vec!["tap-controls"])
            .build();
        container.append(&tap_label);
        container.append(&tap_switch);
        container.append(&double_tap_label);
        container.append(&double_tap_switch);

        let dice = Rc::new(RefCell::new(None::<Dice>));

        let widget = Self {
            container,
            dice,
        };

        widget.connect_signals(tap_switch, double_tap_switch);
        widget
    }

    /// Set the dice to control.
    pub fn set_dice(&self, dice: Dice) {
        *self.dice.borrow_mut() = Some(dice);
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }

    fn connect_signals(&self, tap_switch: gtk4::Switch, double_tap_switch: gtk4::Switch) {
        tap_switch.connect_notify_local(Some("active"), clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            move |switch, _| {
                let enable = switch.is_active();
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        let result = if enable {
                            dice.enable_tap().await
                        } else {
                            dice.disable_tap().await
                        };
                        if let Err(error) = result {
                            debug!(error = %error, "failed to set tap interrupt");
                        }
                    });
                }
            }
        ));

        double_tap_switch.connect_notify_local(Some("active"), clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            move |switch, _| {
                let enable = switch.is_active();
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        let result = if enable {
                            dice.enable_double_tap().await
                        } else {
                            dice.disable_double_tap().await
                        };
                        if let Err(error) = result {
                            debug!(error = %error, "failed to set double tap interrupt");
                        }
                    });
                }
            }
        ));
    }
}

impl Default for TapControls {
    fn default() -> Self {
        Self::new()
    }
}
