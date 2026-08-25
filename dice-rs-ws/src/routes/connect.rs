use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::ws_error::Result;
use crate::ws_error::WsError;

/// Request body for the connect endpoint.
#[derive(Debug, Deserialize)]
pub struct ConnectRequest {
    /// Device MAC address.
    pub address: String,
    /// Optional dice type (e.g. "d6", "d20").
    pub dice_type: Option<String>,
}

/// Response body for the connect endpoint.
#[derive(Debug, Serialize)]
pub struct ConnectResponse {
    /// Session ID for the new connection.
    pub session_id: String,
}

/// POST /api/connect — connect to a GoDice device.
pub async fn connect_handler(State(state): State<Arc<AppState>>, Json(body): Json<ConnectRequest>) -> Result<Json<ConnectResponse>> {
    let device = state.manager.find_device_by_address(&body.address).await?;
    let dice = state.manager.connect(&device).await?;

    if let Some(dice_type) = body.dice_type {
        let dt = dice_rs::model::dice::DiceType::from_str(&dice_type).map_err(|e| WsError::InvalidDiceType(e.to_string()))?;
        dice.set_dice_type(dt);
    }

    let session_id = state.sessions.lock().await.create(dice, body.address);
    Ok(Json(ConnectResponse { session_id }))
}
