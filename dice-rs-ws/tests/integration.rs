use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum_test::TestServer;
use axum_test::WsMessage;
use btleplug::api::BDAddr;
use dice_rs::error::DiceError;
use dice_rs::error::Result;
use dice_rs::service::dice::Dice;
use dice_rs::service::dice::DiceDevice;
use dice_rs::service::manager::DiceService;
use dice_rs_ws::app_state::AppState;
use dice_rs_ws::server::Server;

/// A mock `DiceService` that returns configurable scan results without BLE hardware.
struct MockDiceService {
    devices: Vec<DiceDevice>,
}

impl MockDiceService {
    fn new(devices: Vec<DiceDevice>) -> Self {
        Self { devices }
    }
}

#[async_trait]
impl DiceService for MockDiceService {
    async fn scan(&self) -> Result<Vec<DiceDevice>> {
        Ok(self.devices.clone())
    }

    async fn scan_with_duration(&self, _duration: Duration) -> Result<Vec<DiceDevice>> {
        Ok(self.devices.clone())
    }

    async fn connect(&self, _device: &DiceDevice) -> Result<Dice> {
        Err(DiceError::Ble(dice_rs::ble::ble_error::BleError::Connect(
            "mock: no real device".into(),
        )))
    }

    async fn find_device_by_address(&self, address: &str) -> Result<DiceDevice> {
        self.devices
            .iter()
            .find(|d| d.address.to_string().contains(address))
            .cloned()
            .ok_or_else(|| {
                DiceError::Ble(dice_rs::ble::ble_error::BleError::device_not_found(address))
            })
    }
}

/// Create a `DiceDevice` with the given address and name.
fn make_device(address: &str, name: &str) -> DiceDevice {
    let bytes: [u8; 6] = address
        .split(':')
        .map(|b| u8::from_str_radix(b, 16).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    DiceDevice {
        id: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        address: BDAddr::from(bytes),
        name: name.to_string(),
        rssi: Some(-60),
    }
}

/// Build a `TestServer` with the given mock service.
fn make_test_server(service: MockDiceService) -> TestServer {
    let manager: Arc<dyn DiceService> = Arc::new(service);
    let state = Arc::new(AppState::new(manager));
    let router = Server::build_router(state);
    TestServer::builder()
        .http_transport()
        .build(router)
}

const TEST_ADDRESS: &str = "AA:BB:CC:DD:EE:FF";
const TEST_NAME: &str = "GoDice_001234";

// === REST API Tests ===

#[tokio::test]
async fn scan_endpoint_returns_devices() {
    let device = make_device(TEST_ADDRESS, TEST_NAME);
    let server = make_test_server(MockDiceService::new(vec![device]));

    let response = server.get("/api/scan").await;
    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let devices = body["devices"].as_array().expect("devices array");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0]["name"], TEST_NAME);
    assert_eq!(devices[0]["address"], TEST_ADDRESS);
}

#[tokio::test]
async fn scan_endpoint_with_duration_param() {
    let device = make_device(TEST_ADDRESS, TEST_NAME);
    let server = make_test_server(MockDiceService::new(vec![device]));

    let response = server.get("/api/scan?duration=2").await;
    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body["devices"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn scan_endpoint_empty_results() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let response = server.get("/api/scan").await;
    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body["devices"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn connect_endpoint_device_not_found() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let response = server
        .post("/api/connect")
        .json(&serde_json::json!({
            "address": "AA:BB:CC:DD:EE:FF"
        }))
        .await;

    response.assert_status_not_found();
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], "device_not_found");
}

#[tokio::test]
async fn connect_endpoint_connect_fails() {
    let device = make_device(TEST_ADDRESS, TEST_NAME);
    let server = make_test_server(MockDiceService::new(vec![device]));

    let response = server
        .post("/api/connect")
        .json(&serde_json::json!({
            "address": TEST_ADDRESS
        }))
        .await;

    response.assert_status_internal_server_error();
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], "internal_error");
}

// === WebSocket Tests ===

#[tokio::test]
async fn ws_scan_returns_results() {
    let device = make_device(TEST_ADDRESS, TEST_NAME);
    let server = make_test_server(MockDiceService::new(vec![device]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Scan"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "ScanResults");
    let devices = parsed["devices"].as_array().expect("devices array");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0]["name"], TEST_NAME);
}

#[tokio::test]
async fn ws_scan_empty_results() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Scan"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "ScanResults");
    assert_eq!(parsed["devices"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn ws_invalid_json_returns_error() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket.send_text("not valid json").await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "Error");
    assert_eq!(parsed["code"], "parse_error");
}

#[tokio::test]
async fn ws_connect_device_not_found() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Connect","address":"AA:BB:CC:DD:EE:FF"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "Error");
    assert_eq!(parsed["code"], "device_not_found");
}

#[tokio::test]
async fn ws_connect_fails_with_mock() {
    let device = make_device(TEST_ADDRESS, TEST_NAME);
    let server = make_test_server(MockDiceService::new(vec![device]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Connect","address":"AA:BB:CC:DD:EE:FF"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "Error");
    assert_eq!(parsed["code"], "connect_failed");
}

#[tokio::test]
async fn ws_disconnect_session_not_found() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Disconnect","session_id":"s99"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "Error");
    assert_eq!(parsed["code"], "session_not_found");
}

#[tokio::test]
async fn ws_unknown_action_returns_parse_error() {
    let server = make_test_server(MockDiceService::new(vec![]));

    let mut websocket = server
        .get_websocket("/ws")
        .await
        .into_websocket()
        .await;

    websocket
        .send_text(r#"{"action":"Unknown"}"#)
        .await;

    let message = websocket.receive_message().await;
    let text = match message {
        WsMessage::Text(text) => text.to_string(),
        _ => panic!("expected text message, got {message:?}"),
    };

    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
    assert_eq!(parsed["type"], "Error");
    assert_eq!(parsed["code"], "parse_error");
}
