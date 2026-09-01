//! yahtzee — Kniffel (Yahtzee) game for GoDice, built with GTK 4 and dice-rs.

use std::sync::Arc;

use dice_rs::DiceManager;
use tracing_subscriber::EnvFilter;
use yahtzee::ui::Application;

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
