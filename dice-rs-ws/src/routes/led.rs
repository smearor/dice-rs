use std::str::FromStr;
use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use dice_rs::model::led::LedColor;
use crate::app_state::AppState;
use crate::routes::disconnect::SuccessResponse;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Request body for the LED endpoint.
#[derive(Debug, Deserialize)]
pub struct LedRequest {
    /// Session ID associated with the dice.
    pub session_id: String,
    /// Color as hex string (e.g. "FF0000") or named color.
    pub color: String,
}

/// POST /api/led — set LED color on a connected dice.
pub async fn led_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LedRequest>,
) -> Result<Json<SuccessResponse>> {
    let sessions = state.sessions.lock().await;
    let session = sessions
        .get(&body.session_id)
        .ok_or_else(|| WsError::SessionNotFound(body.session_id.clone()))?;
    let color = LedColor::from_str(&body.color)
        .map_err(|e| WsError::InvalidColor(e.to_string()))?;
    session.dice.set_led(color).await?;
    Ok(Json(SuccessResponse {
        message: "LEDs set".into(),
    }))
}
