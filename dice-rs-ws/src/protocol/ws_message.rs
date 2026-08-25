use serde::Deserialize;
use serde::Serialize;

use dice_rs::model::system_status::SystemStatus;
use dice_rs::service::dice::DiceDevice;
use dice_rs::service::dice::DiceEvent;

/// A message sent from the server to a WebSocket client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// A dice event (roll, stable, tilt, etc.).
    Event {
        /// Session ID associated with the event.
        session_id: String,
        /// The dice event.
        event: DiceEvent,
    },
    /// A successful response to a client request.
    Success {
        /// Session ID associated with the response.
        session_id: String,
        /// Human-readable success message.
        message: String,
    },
    /// An error response.
    Error {
        /// Session ID if applicable.
        session_id: Option<String>,
        /// Machine-readable error code.
        code: String,
        /// Human-readable error message.
        message: String,
    },
    /// Scan results.
    ScanResults {
        /// List of discovered devices.
        devices: Vec<DiceDevice>,
    },
    /// Battery level response.
    BatteryLevel {
        /// Session ID associated with the response.
        session_id: String,
        /// Battery level (0–100 percent).
        level: u8,
    },
    /// System status response.
    SystemStatus {
        /// Session ID associated with the response.
        session_id: String,
        /// System status.
        status: SystemStatus,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use dice_rs::model::acceleration::Acceleration;
    use dice_rs::model::battery_level::BatteryLevel;
    use dice_rs::model::charging_state::ChargingState;
    use dice_rs::model::dice::DiceColor;
    use dice_rs::model::face::FaceValue;

    #[test]
    fn serialize_event_roll_start() {
        let msg = WsMessage::Event {
            session_id: "s1".into(),
            event: DiceEvent::RollStart,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"Event""#));
        assert!(json.contains(r#""session_id":"s1""#));
        assert!(json.contains(r#""kind":"RollStart""#));
    }

    #[test]
    fn serialize_event_stable() {
        let msg = WsMessage::Event {
            session_id: "s1".into(),
            event: DiceEvent::Stable {
                face: FaceValue::new(6).unwrap(),
                acceleration: Acceleration { x: 10, y: -5, z: 3 },
            },
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""kind":"Stable""#));
        assert!(json.contains(r#""face":6"#));
        assert!(json.contains(r#""x":10"#));
    }

    #[test]
    fn serialize_success() {
        let msg = WsMessage::Success {
            session_id: "s1".into(),
            message: "Connected".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"Success""#));
        assert!(json.contains(r#""message":"Connected""#));
    }

    #[test]
    fn serialize_error_with_session() {
        let msg = WsMessage::Error {
            session_id: Some("s1".into()),
            code: "invalid_color".into(),
            message: "bad color".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"Error""#));
        assert!(json.contains(r#""code":"invalid_color""#));
        assert!(json.contains(r#""session_id":"s1""#));
    }

    #[test]
    fn serialize_error_without_session() {
        let msg = WsMessage::Error {
            session_id: None,
            code: "internal".into(),
            message: "oops".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""session_id":null"#));
    }

    #[test]
    fn serialize_scan_results() {
        let json = r#"{"type":"ScanResults","devices":[{"address":"AA:BB:CC:DD:EE:FF","name":"GoDice_001234","rssi":-60}]}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();
        match msg {
            WsMessage::ScanResults { devices } => {
                assert_eq!(devices.len(), 1);
                assert_eq!(devices[0].name, "GoDice_001234");
                assert_eq!(devices[0].rssi, Some(-60));
            }
            _ => panic!("expected ScanResults variant"),
        }
    }

    #[test]
    fn serialize_battery_level() {
        let msg = WsMessage::BatteryLevel {
            session_id: "s1".into(),
            level: 75,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""type":"BatteryLevel""#));
        assert!(json.contains(r#""level":75"#));
    }

    #[test]
    fn deserialize_event_charging() {
        let json = r#"{"type":"Event","session_id":"s2","event":{"kind":"Charging","state":"Charging"}}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();
        match msg {
            WsMessage::Event { session_id, event } => {
                assert_eq!(session_id, "s2");
                assert_eq!(
                    event,
                    DiceEvent::Charging {
                        state: ChargingState::Charging
                    }
                );
            }
            _ => panic!("expected Event variant"),
        }
    }

    #[test]
    fn deserialize_event_tap() {
        let json = r#"{"type":"Event","session_id":"s1","event":{"kind":"Tap"}}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();
        match msg {
            WsMessage::Event { event, .. } => {
                assert_eq!(event, DiceEvent::Tap);
            }
            _ => panic!("expected Event variant"),
        }
    }

    #[test]
    fn deserialize_event_double_tap() {
        let json = r#"{"type":"Event","session_id":"s1","event":{"kind":"DoubleTap"}}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();
        match msg {
            WsMessage::Event { event, .. } => {
                assert_eq!(event, DiceEvent::DoubleTap);
            }
            _ => panic!("expected Event variant"),
        }
    }

    #[test]
    fn deserialize_system_status() {
        let json = r#"{"type":"SystemStatus","session_id":"s1","status":{"battery_level":80,"color":"Green","connected":true,"rssi":-55}}"#;
        let msg: WsMessage = serde_json::from_str(json).unwrap();
        match msg {
            WsMessage::SystemStatus { session_id, status } => {
                assert_eq!(session_id, "s1");
                assert_eq!(status.battery_level, BatteryLevel::new(80));
                assert_eq!(status.color, DiceColor::Green);
                assert!(status.connected);
                assert_eq!(status.rssi, Some(-55));
            }
            _ => panic!("expected SystemStatus variant"),
        }
    }

    #[test]
    fn round_trip_success() {
        let msg = WsMessage::Success {
            session_id: "s1".into(),
            message: "OK".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: WsMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(json, serde_json::to_string(&decoded).unwrap());
    }
}
