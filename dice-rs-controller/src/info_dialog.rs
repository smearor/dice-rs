use gtk4::prelude::*;

use crate::dice_3d::Dice3D;

/// About / info dialog showing app name, spinning dice, links, and license.
pub struct InfoDialog {
    dialog: gtk4::Window,
}

impl InfoDialog {
    /// Create a new info dialog.
    pub fn new(parent: &gtk4::ApplicationWindow) -> Self {
        let dialog = gtk4::Window::builder()
            .title("About dice-rs Controller")
            .modal(true)
            .transient_for(parent)
            .default_width(420)
            .default_height(500)
            .build();

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(24)
            .margin_bottom(24)
            .halign(gtk4::Align::Center)
            .build();

        let app_name = gtk4::Label::builder()
            .label("dice-rs Controller")
            .css_classes(vec!["info-app-name"])
            .build();
        content.append(&app_name);

        let app_desc = gtk4::Label::builder()
            .label("GTK 4 desktop controller for GoDice BLE dice")
            .css_classes(vec!["dim-label"])
            .build();
        content.append(&app_desc);

        let dice_3d = Dice3D::new();
        let dice_3d_frame = gtk4::Frame::builder()
            .css_classes(vec!["dice-3d-frame", "info-dice-frame"])
            .width_request(120)
            .height_request(120)
            .child(dice_3d.widget())
            .build();
        content.append(&dice_3d_frame);

        let links_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(8)
            .halign(gtk4::Align::Center)
            .build();

        let github_link = gtk4::LinkButton::builder()
            .label("GitHub Repository")
            .uri("https://github.com/smearor/dice-rs")
            .build();
        links_box.append(&github_link);

        let docs_link = gtk4::LinkButton::builder()
            .label("docs.rs Documentation")
            .uri("https://docs.rs/dice-rs")
            .build();
        links_box.append(&docs_link);

        let particula_link = gtk4::LinkButton::builder()
            .label("Particula Tech - GoDice")
            .uri("https://particula-tech.com/pages/godice")
            .build();
        links_box.append(&particula_link);

        content.append(&links_box);

        let license_label = gtk4::Label::builder()
            .label("Licensed under the MIT License")
            .css_classes(vec!["dim-label", "info-license"])
            .build();
        content.append(&license_label);

        dialog.set_child(Some(&content));

        Self { dialog }
    }

    /// Present the dialog.
    pub fn present(&self) {
        self.dialog.present();
    }
}
