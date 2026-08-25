use std::sync::Arc;
use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use dice_rs::model::system_status::SystemStatus;
use crate::app_state::AppState;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Query parameters for the status endpoint.
#[derive(Debug, Deserialize)]
pub struct StatusParams {
    /// Session ID associated with the dice.
    pub session_id: String,
}

/// GET /api/status — query system status of a connected dice.
pub async fn status_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<StatusParams>,
) -> Result<Json<SystemStatus>> {
    let sessions = state.sessions.lock().await;
    let session = sessions
        .get(&params.session_id)
        .ok_or_else(|| WsError::SessionNotFound(params.session_id.clone()))?;
    let status = session.dice.system_status().await?;
    Ok(Json(status))
}
