use std::sync::Arc;

use dice_rs::DiceManager;
use gtk4::prelude::*;

use crate::ui::styling::load_css;
use crate::ui::window::MainWindow;

/// The GTK application for the Yahtzee game.
pub struct Application {
    /// The dice-rs BLE manager for scanning and connecting dice.
    manager: Arc<DiceManager>,
}

impl Application {
    /// Create a new application instance.
    pub fn new(manager: Arc<DiceManager>) -> Self {
        Self { manager }
    }

    /// Run the application.
    ///
    /// This method blocks until the GTK application exits.
    pub fn run(&self) {
        let app = gtk4::Application::builder().application_id("io.github.smearor.dice-rs.yahtzee").build();

        app.connect_startup(|_| {
            load_css();
        });

        let manager = self.manager.clone();
        app.connect_activate(move |gtk_app| {
            let window = MainWindow::new(gtk_app, manager.clone());
            window.present();
        });

        app.run();

        // Ensure the process exits when the GTK app returns — the tokio runtime
        // and BLE background tasks would otherwise keep it alive.
        std::process::exit(0);
    }
}
