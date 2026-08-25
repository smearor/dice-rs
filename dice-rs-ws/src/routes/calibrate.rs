use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::routes::disconnect::SuccessResponse;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Request body for the calibrate endpoint.
#[derive(Debug, Deserialize)]
pub struct CalibrateRequest {
    /// Session ID associated with the dice.
    pub session_id: String,
}

/// POST /api/calibrate — calibrate the sensor of a connected dice.
pub async fn calibrate_handler(State(state): State<Arc<AppState>>, Json(body): Json<CalibrateRequest>) -> Result<Json<SuccessResponse>> {
    let sessions = state.sessions.lock().await;
    let session = sessions
        .get(&body.session_id)
        .ok_or_else(|| WsError::SessionNotFound(body.session_id.clone()))?;
    session.dice.calibrate().await?;
    Ok(Json(SuccessResponse {
        message: "Calibration complete".into(),
    }))
}
