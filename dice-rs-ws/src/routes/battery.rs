use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Query parameters for the battery endpoint.
#[derive(Debug, Deserialize)]
pub struct BatteryParams {
    /// Session ID associated with the dice.
    pub session_id: String,
}

/// Response body for the battery endpoint.
#[derive(Debug, Serialize)]
pub struct BatteryResponse {
    /// Battery level (0–100 percent).
    pub battery_level: u8,
}

/// GET /api/battery — query battery level of a connected dice.
pub async fn battery_handler(State(state): State<Arc<AppState>>, Query(params): Query<BatteryParams>) -> Result<Json<BatteryResponse>> {
    let sessions = state.sessions.lock().await;
    let session = sessions
        .get(&params.session_id)
        .ok_or_else(|| WsError::SessionNotFound(params.session_id.clone()))?;
    let level = session.dice.get_battery_level().await?;
    Ok(Json(BatteryResponse { battery_level: level.into() }))
}
