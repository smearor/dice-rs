//! dice-rs-controller - GTK 4 desktop controller application for GoDice.

mod application;
mod app_settings;
mod battery_indicator;
mod battery_level_style;
mod config_dir;
mod dice_3d;
mod dice_settings;
mod dice_style;
mod dice_renderer;
mod dice_row;
mod dice_type_icon;
mod event_controller;
mod face_display;
mod info_dialog;
mod led_controls;
mod models;
mod orientation_state;
mod roll_history;
mod settings_dialog;
mod stability_style;
mod tap_controls;
mod tap_indicator;
mod window;


use std::sync::Arc;

use dice_rs::service::manager::DiceManager;
use tracing_subscriber::EnvFilter;

use crate::application::Application;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,bluez_async=warn")
    })).init();

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
