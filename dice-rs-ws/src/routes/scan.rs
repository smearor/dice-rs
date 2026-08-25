use std::sync::Arc;
use std::time::Duration;

use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use dice_rs::service::dice::DiceDevice;
use dice_rs::service::manager::DiceManager;
use crate::app_state::AppState;
use crate::ws_error::Result;
use crate::ws_error::WsError;

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
    let scanner = state.manager.scanner().with_scan_duration(duration);
    let devices = scanner.scan().await?;
    Ok(Json(ScanResponse {
        devices,
    }))
}

/// Find a device by MAC address from scan results.
pub async fn find_device_by_address(
    manager: &DiceManager,
    address: &str,
) -> Result<DiceDevice> {
    let devices = manager.scan().await?;
    devices
        .into_iter()
        .find(|d| d.address.to_string().contains(address))
        .ok_or_else(|| WsError::DeviceNotFound(address.to_string()))
}
