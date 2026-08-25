use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use serde::Deserialize;

/// A request received from a WebSocket client.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action")]
pub enum WsRequest {
    /// Scan for devices.
    Scan {
        /// Optional scan duration in seconds.
        duration: Option<u64>,
    },
    /// Connect to a device.
    Connect {
        /// Device MAC address.
        address: String,
        /// Optional dice type (e.g. "d6", "d20").
        dice_type: Option<String>,
    },
    /// Disconnect from a device.
    Disconnect {
        /// Session ID to disconnect.
        session_id: String,
    },
    /// Set LED color.
    SetLed {
        /// Session ID associated with the dice.
        session_id: String,
        /// Color as hex string (e.g. "FF0000") or named color.
        color: String,
    },
    /// Pulse LEDs.
    PulseLed {
        /// Session ID associated with the dice.
        session_id: String,
        /// Color as hex string or named color.
        color: String,
        /// Number of pulse cycles.
        count: u8,
        /// On time in 10ms units.
        on_time: u8,
        /// Off time in 10ms units.
        off_time: u8,
        /// Blink mode: "rainbow" or "color".
        #[serde(default)]
        blink_mode: PulseBlinkMode,
        /// LED selection: "both", "led1", or "led2".
        #[serde(default)]
        leds: PulseLeds,
    },
    /// Turn LEDs off.
    TurnOffLeds {
        /// Session ID associated with the dice.
        session_id: String,
    },
    /// Query battery level.
    GetBattery {
        /// Session ID associated with the dice.
        session_id: String,
    },
    /// Query system status.
    GetStatus {
        /// Session ID associated with the dice.
        session_id: String,
    },
    /// Calibrate sensor.
    Calibrate {
        /// Session ID associated with the dice.
        session_id: String,
    },
    /// Enable or disable single tap interrupt notifications.
    SetTapInterrupt {
        /// Session ID associated with the dice.
        session_id: String,
        /// Enable (true) or disable (false) tap notifications.
        enable: bool,
    },
    /// Enable or disable double tap interrupt notifications.
    SetDoubleTapInterrupt {
        /// Session ID associated with the dice.
        session_id: String,
        /// Enable (true) or disable (false) double tap notifications.
        enable: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_scan() {
        let json = r#"{"action":"Scan","duration":10}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Scan { duration } => assert_eq!(duration, Some(10)),
            _ => panic!("expected Scan variant"),
        }
    }

    #[test]
    fn deserialize_scan_no_duration() {
        let json = r#"{"action":"Scan"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Scan { duration } => assert_eq!(duration, None),
            _ => panic!("expected Scan variant"),
        }
    }

    #[test]
    fn deserialize_connect() {
        let json = r#"{"action":"Connect","address":"AA:BB:CC:DD:EE:FF","dice_type":"d6"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Connect { address, dice_type } => {
                assert_eq!(address, "AA:BB:CC:DD:EE:FF");
                assert_eq!(dice_type, Some("d6".into()));
            }
            _ => panic!("expected Connect variant"),
        }
    }

    #[test]
    fn deserialize_connect_no_dice_type() {
        let json = r#"{"action":"Connect","address":"AA:BB:CC:DD:EE:FF"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Connect { dice_type, .. } => assert_eq!(dice_type, None),
            _ => panic!("expected Connect variant"),
        }
    }

    #[test]
    fn deserialize_disconnect() {
        let json = r#"{"action":"Disconnect","session_id":"s1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Disconnect { session_id } => assert_eq!(session_id, "s1"),
            _ => panic!("expected Disconnect variant"),
        }
    }

    #[test]
    fn deserialize_set_led() {
        let json = r#"{"action":"SetLed","session_id":"s1","color":"FF0000"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::SetLed { session_id, color } => {
                assert_eq!(session_id, "s1");
                assert_eq!(color, "FF0000");
            }
            _ => panic!("expected SetLed variant"),
        }
    }

    #[test]
    fn deserialize_pulse_led_with_defaults() {
        let json = r#"{"action":"PulseLed","session_id":"s1","color":"00FF00","count":5,"on_time":10,"off_time":10}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::PulseLed { session_id, color, count, on_time, off_time, blink_mode, leds } => {
                assert_eq!(session_id, "s1");
                assert_eq!(color, "00FF00");
                assert_eq!(count, 5);
                assert_eq!(on_time, 10);
                assert_eq!(off_time, 10);
                assert_eq!(blink_mode, PulseBlinkMode::Color);
                assert_eq!(leds, PulseLeds::Both);
            }
            _ => panic!("expected PulseLed variant"),
        }
    }

    #[test]
    fn deserialize_pulse_led_with_blink_mode_and_leds() {
        let json = r#"{"action":"PulseLed","session_id":"s1","color":"00FF00","count":3,"on_time":5,"off_time":5,"blink_mode":"Rainbow","leds":"Led1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::PulseLed { blink_mode, leds, .. } => {
                assert_eq!(blink_mode, PulseBlinkMode::Rainbow);
                assert_eq!(leds, PulseLeds::Led1);
            }
            _ => panic!("expected PulseLed variant"),
        }
    }

    #[test]
    fn deserialize_turn_off_leds() {
        let json = r#"{"action":"TurnOffLeds","session_id":"s1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::TurnOffLeds { session_id } => assert_eq!(session_id, "s1"),
            _ => panic!("expected TurnOffLeds variant"),
        }
    }

    #[test]
    fn deserialize_get_battery() {
        let json = r#"{"action":"GetBattery","session_id":"s1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::GetBattery { session_id } => assert_eq!(session_id, "s1"),
            _ => panic!("expected GetBattery variant"),
        }
    }

    #[test]
    fn deserialize_get_status() {
        let json = r#"{"action":"GetStatus","session_id":"s1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::GetStatus { session_id } => assert_eq!(session_id, "s1"),
            _ => panic!("expected GetStatus variant"),
        }
    }

    #[test]
    fn deserialize_calibrate() {
        let json = r#"{"action":"Calibrate","session_id":"s1"}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::Calibrate { session_id } => assert_eq!(session_id, "s1"),
            _ => panic!("expected Calibrate variant"),
        }
    }

    #[test]
    fn deserialize_set_tap_interrupt() {
        let json = r#"{"action":"SetTapInterrupt","session_id":"s1","enable":true}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::SetTapInterrupt { session_id, enable } => {
                assert_eq!(session_id, "s1");
                assert!(enable);
            }
            _ => panic!("expected SetTapInterrupt variant"),
        }
    }

    #[test]
    fn deserialize_set_double_tap_interrupt() {
        let json = r#"{"action":"SetDoubleTapInterrupt","session_id":"s1","enable":false}"#;
        let req: WsRequest = serde_json::from_str(json).unwrap();
        match req {
            WsRequest::SetDoubleTapInterrupt { session_id, enable } => {
                assert_eq!(session_id, "s1");
                assert!(!enable);
            }
            _ => panic!("expected SetDoubleTapInterrupt variant"),
        }
    }

    #[test]
    fn deserialize_unknown_action_fails() {
        let json = r#"{"action":"Unknown","session_id":"s1"}"#;
        assert!(serde_json::from_str::<WsRequest>(json).is_err());
    }
}
