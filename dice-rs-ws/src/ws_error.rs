use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use dice_rs::ble::ble_error::BleError;
use dice_rs::error::DiceError;
use thiserror::Error;

/// Errors returned by the WebSocket server.
#[derive(Debug, Error)]
pub enum WsError {
    /// Session ID not found.
    #[error("session not found: {0}")]
    SessionNotFound(String),

    /// Device address not found in scan results.
    #[error("device not found: {0}")]
    DeviceNotFound(String),

    /// Invalid color string.
    #[error("invalid color: {0}")]
    InvalidColor(String),

    /// Invalid dice type string.
    #[error("invalid dice type: {0}")]
    InvalidDiceType(String),

    /// Underlying dice-rs library error.
    #[error(transparent)]
    Dice(DiceError),

    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error (e.g. network bind failure).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for WsError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            WsError::SessionNotFound(_) => (StatusCode::NOT_FOUND, "session_not_found", self.to_string()),
            WsError::DeviceNotFound(_) => (StatusCode::NOT_FOUND, "device_not_found", self.to_string()),
            WsError::InvalidColor(_) | WsError::InvalidDiceType(_) => (StatusCode::BAD_REQUEST, "invalid_input", self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", self.to_string()),
        };
        let body = serde_json::json!({ "code": code, "message": message });
        (status, Json(body)).into_response()
    }
}

/// Result type alias for the WebSocket server.
pub type Result<T> = std::result::Result<T, WsError>;

impl From<DiceError> for WsError {
    fn from(err: DiceError) -> Self {
        match &err {
            DiceError::Ble(BleError::DeviceNotFound { address }) => WsError::DeviceNotFound(address.clone()),
            _ => WsError::Dice(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use dice_rs::ble::ble_error::BleError;

    #[tokio::test]
    async fn session_not_found_returns_404() {
        let err = WsError::SessionNotFound("s1".into());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "session_not_found");
    }

    #[tokio::test]
    async fn device_not_found_returns_404() {
        let err = WsError::DeviceNotFound("AA:BB".into());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "device_not_found");
    }

    #[tokio::test]
    async fn invalid_color_returns_400() {
        let err = WsError::InvalidColor("bad".into());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "invalid_input");
    }

    #[tokio::test]
    async fn invalid_dice_type_returns_400() {
        let err = WsError::InvalidDiceType("bad".into());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn dice_error_returns_500() {
        let err = WsError::Dice(DiceError::Ble(BleError::Connect("addr".into())));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "internal_error");
    }

    #[tokio::test]
    async fn json_error_returns_500() {
        let err = WsError::Json(serde_json::from_str::<serde_json::Value>("bad").unwrap_err());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn io_error_returns_500() {
        let err = WsError::Io(std::io::Error::new(std::io::ErrorKind::AddrInUse, "addr in use"));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
