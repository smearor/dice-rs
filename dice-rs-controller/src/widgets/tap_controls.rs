use crate::platform::widget_container::WidgetContainer;
use crate::services::dice_service::DiceService;
use gtk4::prelude::*;

/// Tap and double tap interrupt controls for a connected dice.
pub struct TapControls {
    container: gtk4::Box,
    dice_service: DiceService,
}

impl TapControls {
    /// Create a new tap controls panel.
    pub fn new(dice_service: DiceService) -> Self {
        let tap_label = gtk4::Label::builder().label("Tap").build();
        let tap_switch = gtk4::Switch::builder().tooltip_text("Enable / disable tap notifications").build();

        let double_tap_label = gtk4::Label::builder().label("Double Tap").build();
        let double_tap_switch = gtk4::Switch::builder().tooltip_text("Enable / disable double tap notifications").build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .css_classes(vec!["tap-controls"])
            .margin_end(12)
            .build();
        container.append(&tap_label);
        container.append(&tap_switch);
        container.append(&double_tap_label);
        container.append(&double_tap_switch);

        let widget = Self { container, dice_service };

        widget.connect_signals(tap_switch, double_tap_switch);
        widget
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }

    fn connect_signals(&self, tap_switch: gtk4::Switch, double_tap_switch: gtk4::Switch) {
        let dice_service = self.dice_service.clone();
        tap_switch.connect_notify_local(Some("active"), move |switch, _| {
            if switch.is_active() {
                dice_service.enable_tap();
            } else {
                dice_service.disable_tap();
            }
        });

        let dice_service = self.dice_service.clone();
        double_tap_switch.connect_notify_local(Some("active"), move |switch, _| {
            if switch.is_active() {
                dice_service.enable_double_tap();
            } else {
                dice_service.disable_double_tap();
            }
        });
    }
}

impl WidgetContainer for TapControls {
    fn widget(&self) -> &gtk4::Widget {
        self.container.as_ref()
    }
}
