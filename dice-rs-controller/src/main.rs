//! dice-rs-controller - GTK 4 desktop controller application for GoDice.

mod application;
mod config;
mod info_dialog;
mod models;
mod orientation_state;
mod platform;
mod services;
mod settings_dialog;
mod styling;
mod widgets;
mod window;

use std::sync::Arc;

use dice_rs::service::manager::DiceManager;
use tracing_subscriber::EnvFilter;

use crate::application::Application;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,bluez_async=warn")))
        .init();

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
