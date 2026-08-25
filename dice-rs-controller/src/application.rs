use std::sync::Arc;

use dice_rs::service::manager::DiceManager;
use gtk4::prelude::*;

use crate::window::MainWindow;

/// The GTK application for the dice-rs controller.
pub struct Application {
    manager: Arc<DiceManager>,
}

impl Application {
    /// Create a new application instance.
    pub fn new(manager: Arc<DiceManager>) -> Self {
        Self { manager }
    }

    /// Run the application.
    pub fn run(&self) {
        let app = gtk4::Application::builder().application_id("io.github.smearor.dice-rs").build();

        app.connect_startup(|_gtk_app| {
            let css = include_str!("../resources/style.css");
            let provider = gtk4::CssProvider::new();
            provider.load_from_data(css);
            if let Some(display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(
                    &display,
                    &provider,
                    gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
        });

        let manager = self.manager.clone();
        app.connect_activate(move |gtk_app| {
            let window = MainWindow::new(gtk_app, manager.clone());
            window.present();
        });

        app.run();

        // Ensure the process exits when the GTK app returns - the tokio runtime
        // and BLE background tasks would otherwise keep it alive.
        std::process::exit(0);
    }
}
