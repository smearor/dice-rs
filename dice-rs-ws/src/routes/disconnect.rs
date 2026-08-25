use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::app_state::AppState;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Request body for the disconnect endpoint.
#[derive(Debug, Deserialize)]
pub struct DisconnectRequest {
    /// Session ID to disconnect.
    pub session_id: String,
}

/// Response body for the disconnect endpoint.
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    /// Human-readable success message.
    pub message: String,
}

/// POST /api/disconnect — disconnect from a GoDice device.
pub async fn disconnect_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<DisconnectRequest>,
) -> Result<Json<SuccessResponse>> {
    let session = state
        .sessions
        .lock()
        .await
        .remove(&body.session_id)
        .ok_or_else(|| WsError::SessionNotFound(body.session_id.clone()))?;
    session.dice.disconnect().await?;
    Ok(Json(SuccessResponse {
        message: "Disconnected".into(),
    }))
}
