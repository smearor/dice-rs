use std::str::FromStr;

use dice_rs::model::dice::DiceType;
use gtk4::prelude::*;
use tracing::debug;

use crate::platform::widget_container::WidgetContainer;
use crate::services::dice_service::DiceService;
use crate::styling::dice_type_icon::create_icon;
use crate::widgets::dice_3d::Dice3D;

/// Dropdown selector for choosing a dice shell type (D6, D20, etc.).
///
/// Displays isometric 3D dice icons in the dropdown items and persists
/// the selected type to per-dice config on change.
pub struct DiceTypeSelector {
    dropdown: gtk4::DropDown,
}

impl DiceTypeSelector {
    /// Create a new dice type selector for the given dice.
    ///
    /// The selector is initialized with the dice's current type and
    /// wired to update the dice, 3D renderer, and config on change.
    pub fn new(dice_service: &DiceService, dice_3d: &Dice3D) -> Self {
        let dice_types = DiceType::sorted_by_count();
        let dice_type_labels: Vec<String> = dice_types.iter().map(|t| t.to_string()).collect();
        let dice_type_model = gtk4::StringList::new(&dice_type_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let dropdown = gtk4::DropDown::builder()
            .model(&dice_type_model)
            .tooltip_text("Dice shell type")
            .css_classes(vec!["dice-type-selector"])
            .build();

        // Custom factory: render isometric dice icons in dropdown items.
        let factory = gtk4::SignalListItemFactory::new();
        let types_for_setup = dice_types.clone();
        factory.connect_setup(move |_item, list_item| {
            let overlay = gtk4::Overlay::builder().css_classes(vec!["dice-type-item"]).build();
            list_item.set_child(Some(&overlay));
        });
        factory.connect_bind(move |_item, list_item| {
            let position = list_item.position() as usize;
            let dice_type = types_for_setup[position % types_for_setup.len()];
            let icon = create_icon(dice_type);
            icon.set_hexpand(true);
            icon.set_vexpand(true);
            icon.set_halign(gtk4::Align::Center);
            icon.set_valign(gtk4::Align::Center);
            let label = gtk4::Label::builder()
                .label(dice_type.to_string())
                .css_classes(vec!["dice-type-label"])
                .halign(gtk4::Align::Center)
                .valign(gtk4::Align::Center)
                .build();
            let overlay = list_item.child().and_downcast::<gtk4::Overlay>().expect("child is Overlay");
            overlay.set_child(Some(&icon));
            overlay.add_overlay(&label);
        });
        dropdown.set_factory(Some(&factory));

        // Square size matching face display.
        dropdown.set_size_request(80, 80);
        dropdown.set_margin_top(8);
        dropdown.set_margin_end(8);

        // Select the current dice type.
        let current_type = dice_service.dice().dice_type();
        if let Some(pos) = dice_types.iter().position(|t| *t == current_type) {
            dropdown.set_selected(pos as u32);
        }

        let dice_service_for_type = dice_service.clone();
        let dice_3d_for_type = dice_3d.clone();
        dropdown.connect_notify_local(Some("selected"), move |dd, _pspec| {
            let Some(item) = dd.selected_item() else {
                return;
            };
            let Some(text) = item.downcast::<gtk4::StringObject>().ok() else {
                return;
            };
            match DiceType::from_str(text.string().as_str()) {
                Ok(dt) => {
                    dice_service_for_type.set_dice_type(dt);
                    dice_3d_for_type.set_dice_type(dt);
                }
                Err(error) => debug!(error = %error, "invalid dice type selected"),
            }
        });

        Self { dropdown }
    }

    /// Returns the dropdown widget for packing.
    pub fn widget(&self) -> &gtk4::DropDown {
        &self.dropdown
    }
}

impl WidgetContainer for DiceTypeSelector {
    fn widget(&self) -> &gtk4::Widget {
        self.dropdown.as_ref()
    }
}
