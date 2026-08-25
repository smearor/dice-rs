//! dice-rs-controller — GTK 4 desktop controller application for GoDice.

mod application;
mod battery_indicator;
mod dice_3d;
mod dice_model;
mod dice_renderer;
mod dice_row;
mod event_controller;
mod face_display;
mod led_controls;
mod tap_indicator;
mod window;

use std::sync::Arc;

use dice_rs::service::manager::DiceManager;

use crate::application::Application;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let manager = match DiceManager::new().await {
        Ok(manager) => Arc::new(manager),
        Err(error) => {
            tracing::error!(error = %error, "failed to create DiceManager");
            return;
        }
    };

    let app = Application::new(manager);
    app.run();
}
