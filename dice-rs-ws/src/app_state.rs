use std::sync::Arc;

use dice_rs::service::manager::DiceService;
use tokio::sync::Mutex;

use crate::session_manager::SessionManager;

/// Shared application state for the WebSocket server.
pub struct AppState {
    /// The dice service for BLE operations.
    pub manager: Arc<dyn DiceService>,
    /// Active sessions keyed by session ID.
    pub sessions: Arc<Mutex<SessionManager>>,
}

impl AppState {
    /// Create new application state.
    pub fn new(manager: Arc<dyn DiceService>) -> Self {
        Self {
            manager,
            sessions: Arc::new(Mutex::new(SessionManager::new())),
        }
    }
}
