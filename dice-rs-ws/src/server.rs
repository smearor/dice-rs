use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use axum::routing::post;

use crate::app_state::AppState;
use crate::routes;
use crate::ws_error::Result;
use crate::ws_handler::handle_ws_upgrade;

/// The dice-rs WebSocket server.
pub struct Server {
    router: Router,
    bind_address: SocketAddr,
}

impl Server {
    /// Build the axum router with all WebSocket and REST API routes.
    pub fn build_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/ws", get(handle_ws_upgrade))
            .route("/api/scan", get(routes::scan_handler))
            .route("/api/connect", post(routes::connect_handler))
            .route("/api/disconnect", post(routes::disconnect_handler))
            .route("/api/led", post(routes::led_handler))
            .route("/api/battery", get(routes::battery_handler))
            .route("/api/status", get(routes::status_handler))
            .route("/api/calibrate", post(routes::calibrate_handler))
            .with_state(state)
    }

    /// Create a new server with the given application state.
    pub fn new(state: Arc<AppState>, bind_address: SocketAddr) -> Self {
        Self {
            router: Self::build_router(state),
            bind_address,
        }
    }

    /// Start the server.
    pub async fn run(self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(self.bind_address).await?;
        tracing::info!(address = %self.bind_address, "WebSocket server listening");
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
