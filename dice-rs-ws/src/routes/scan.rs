use std::sync::Arc;
use std::time::Duration;

use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use dice_rs::service::dice::DiceDevice;
use crate::app_state::AppState;
use crate::ws_error::Result;

/// Query parameters for the scan endpoint.
#[derive(Debug, Deserialize)]
pub struct ScanParams {
    /// Optional scan duration in seconds (default: 5).
    pub duration: Option<u64>,
}

/// Response body for the scan endpoint.
#[derive(Debug, Serialize)]
pub struct ScanResponse {
    /// List of discovered devices.
    pub devices: Vec<DiceDevice>,
}

/// GET /api/scan — scan for GoDice devices.
pub async fn scan_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ScanParams>,
) -> Result<Json<ScanResponse>> {
    let duration = Duration::from_secs(params.duration.unwrap_or(5));
    let devices = state.manager.scan_with_duration(duration).await?;
    Ok(Json(ScanResponse {
        devices,
    }))
}
