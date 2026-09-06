use crate::i18n;
use gtk4::prelude::*;

/// About / info dialog showing app name, description, and license.
pub struct InfoDialog {
    /// The GTK window.
    dialog: gtk4::Window,
}

impl InfoDialog {
    /// Create a new info dialog.
    pub fn new(parent: &gtk4::ApplicationWindow) -> Self {
        let dialog = gtk4::Window::builder()
            .title(&i18n::get("info-dialog-title"))
            .modal(true)
            .transient_for(parent)
            .default_width(420)
            .default_height(400)
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
            .label(&i18n::get("info-app-name"))
            .css_classes(vec!["info-app-name"])
            .build();
        content.append(&app_name);

        let app_desc = gtk4::Label::builder()
            .label(&i18n::get("info-app-description"))
            .css_classes(vec!["dim-label"])
            .build();
        content.append(&app_desc);

        let links_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(8)
            .halign(gtk4::Align::Center)
            .build();

        let github_link = gtk4::LinkButton::builder()
            .label(&i18n::get("info-github"))
            .uri("https://github.com/smearor/dice-rs")
            .build();
        links_box.append(&github_link);

        let docs_link = gtk4::LinkButton::builder()
            .label(&i18n::get("info-docs"))
            .uri("https://docs.rs/dice-rs")
            .build();
        links_box.append(&docs_link);

        let particula_link = gtk4::LinkButton::builder()
            .label(&i18n::get("info-particula"))
            .uri("https://particula-tech.com/pages/godice")
            .build();
        links_box.append(&particula_link);

        content.append(&links_box);

        let license_label = gtk4::Label::builder()
            .label(&i18n::get("info-license"))
            .css_classes(vec!["dim-label", "info-license"])
            .build();
        content.append(&license_label);

        let close_button = gtk4::Button::builder()
            .label(&i18n::get("info-close"))
            .css_classes(vec!["suggested-action"])
            .halign(gtk4::Align::Center)
            .build();
        content.append(&close_button);

        {
            let dialog_clone = dialog.clone();
            close_button.connect_clicked(move |_| {
                dialog_clone.close();
            });
        }

        dialog.set_child(Some(&content));

        Self { dialog }
    }

    /// Present the dialog.
    pub fn present(&self) {
        self.dialog.present();
    }
}
