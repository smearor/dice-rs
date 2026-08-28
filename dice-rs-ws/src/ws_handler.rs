use crate::app_state::AppState;
use crate::protocol::WsMessage;
use crate::protocol::WsRequest;
use axum::extract::State;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::Response;
use dice_rs::model::led::LedColor;
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use futures::SinkExt;
use futures::StreamExt;
use tokio::sync::Mutex;
use tracing::debug;
use tracing::error;

use std::str::FromStr;
use std::sync::Arc;

/// Type alias for the WebSocket sender half.
type WsSender = Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>;

/// Handle the WebSocket upgrade from an HTTP request.
pub async fn handle_ws_upgrade(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(move |socket| run(socket, state))
}

/// Main loop for a single WebSocket connection.
async fn run(socket: WebSocket, state: Arc<AppState>) {
    let (sender, mut receiver) = socket.split();
    let sender: WsSender = Arc::new(Mutex::new(sender));

    loop {
        match receiver.next().await {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<WsRequest>(&text) {
                Ok(request) => {
                    handle_request(request, &state, &sender).await;
                }
                Err(err) => {
                    send_error(&sender, None, "parse_error", &err.to_string()).await;
                }
            },
            Some(Ok(Message::Close(_))) | None => break,
            _ => {}
        }
    }
}

/// Handle a single client request.
async fn handle_request(request: WsRequest, state: &Arc<AppState>, sender: &WsSender) {
    match request {
        WsRequest::Scan { duration } => {
            handle_scan(state, sender, duration).await;
        }
        WsRequest::Connect { address, dice_type } => {
            handle_connect(state, sender, address, dice_type).await;
        }
        WsRequest::Disconnect { session_id } => {
            handle_disconnect(state, sender, session_id).await;
        }
        WsRequest::SetLed { session_id, color } => {
            handle_set_led(state, sender, session_id, color).await;
        }
        WsRequest::PulseLed {
            session_id,
            color,
            count,
            on_time,
            off_time,
            blink_mode,
            leds,
        } => {
            handle_pulse_led(state, sender, session_id, color, count, on_time, off_time, blink_mode, leds).await;
        }
        WsRequest::TurnOffLeds { session_id } => {
            handle_turn_off_leds(state, sender, session_id).await;
        }
        WsRequest::GetBattery { session_id } => {
            handle_get_battery(state, sender, session_id).await;
        }
        WsRequest::GetStatus { session_id } => {
            handle_get_status(state, sender, session_id).await;
        }
        WsRequest::Calibrate { session_id } => {
            handle_calibrate(state, sender, session_id).await;
        }
        WsRequest::SetTapInterrupt { session_id, enable } => {
            handle_set_tap_interrupt(state, sender, session_id, enable).await;
        }
        WsRequest::SetDoubleTapInterrupt { session_id, enable } => {
            handle_set_double_tap_interrupt(state, sender, session_id, enable).await;
        }
    }
}

async fn handle_scan(state: &Arc<AppState>, sender: &WsSender, _duration: Option<u64>) {
    let result = state.manager.scan().await;
    match result {
        Ok(devices) => {
            let msg = WsMessage::ScanResults { devices };
            send_message(sender, &msg).await;
        }
        Err(err) => {
            send_error(sender, None, "scan_failed", &err.to_string()).await;
        }
    }
}

async fn handle_connect(state: &Arc<AppState>, sender: &WsSender, address: String, dice_type: Option<String>) {
    let device = match state.manager.find_device_by_address(&address).await {
        Ok(d) => d,
        Err(err) => {
            send_error(sender, None, "device_not_found", &err.to_string()).await;
            return;
        }
    };

    let dice = match state.manager.connect(&device).await {
        Ok(d) => d,
        Err(err) => {
            send_error(sender, None, "connect_failed", &err.to_string()).await;
            return;
        }
    };

    if let Some(dt) = dice_type
        && let Ok(parsed) = dice_rs::model::dice::DiceType::from_str(&dt)
    {
        dice.set_dice_type(parsed);
    }

    let session_id = state.sessions.lock().await.create(dice.clone(), address.clone());

    let msg = WsMessage::Success {
        session_id: session_id.clone(),
        message: "Connected".into(),
    };
    send_message(sender, &msg).await;

    // Spawn event streaming task for this session.
    let sender_clone = sender.clone();
    let session_id_clone = session_id.clone();
    let mut event_receiver = dice.subscribe();
    tokio::spawn(async move {
        loop {
            match event_receiver.recv().await {
                Ok(event) => {
                    let ws_msg = WsMessage::Event {
                        session_id: session_id_clone.clone(),
                        event,
                    };
                    if !send_message(&sender_clone, &ws_msg).await {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    debug!("WebSocket event stream missed {n} events");
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    });
}

async fn handle_disconnect(state: &Arc<AppState>, sender: &WsSender, session_id: String) {
    let session = state.sessions.lock().await.remove(&session_id);
    match session {
        Some(s) => {
            if let Err(err) = s.dice.disconnect().await {
                send_error(sender, Some(&session_id), "disconnect_failed", &err.to_string()).await;
                return;
            }
            let msg = WsMessage::Success {
                session_id,
                message: "Disconnected".into(),
            };
            send_message(sender, &msg).await;
        }
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
        }
    }
}

async fn handle_set_led(state: &Arc<AppState>, sender: &WsSender, session_id: String, color: String) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    let parsed_color = match LedColor::from_str(&color) {
        Ok(c) => c,
        Err(e) => {
            send_error(sender, Some(&session_id), "invalid_color", &e.to_string()).await;
            return;
        }
    };

    if let Err(err) = session.dice.set_led(parsed_color).await {
        send_error(sender, Some(&session_id), "led_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: "LEDs set".into(),
    };
    send_message(sender, &msg).await;
}

#[allow(clippy::too_many_arguments)]
async fn handle_pulse_led(
    state: &Arc<AppState>,
    sender: &WsSender,
    session_id: String,
    color: String,
    count: u8,
    on_time: u8,
    off_time: u8,
    blink_mode: PulseBlinkMode,
    leds: PulseLeds,
) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    let parsed_color = match LedColor::from_str(&color) {
        Ok(c) => c,
        Err(e) => {
            send_error(sender, Some(&session_id), "invalid_color", &e.to_string()).await;
            return;
        }
    };

    if let Err(err) = session.dice.pulse_leds(count, on_time, off_time, parsed_color, blink_mode, leds).await {
        send_error(sender, Some(&session_id), "pulse_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: "LEDs pulsed".into(),
    };
    send_message(sender, &msg).await;
}

async fn handle_turn_off_leds(state: &Arc<AppState>, sender: &WsSender, session_id: String) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    if let Err(err) = session.dice.turn_off_leds().await {
        send_error(sender, Some(&session_id), "led_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: "LEDs off".into(),
    };
    send_message(sender, &msg).await;
}

async fn handle_get_battery(state: &Arc<AppState>, sender: &WsSender, session_id: String) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    match session.dice.get_battery_level().await {
        Ok(level) => {
            let msg = WsMessage::BatteryLevel {
                session_id,
                level: level.into(),
            };
            send_message(sender, &msg).await;
        }
        Err(err) => {
            send_error(sender, Some(&session_id), "battery_failed", &err.to_string()).await;
        }
    }
}

async fn handle_get_status(state: &Arc<AppState>, sender: &WsSender, session_id: String) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    match session.dice.system_status().await {
        Ok(status) => {
            let msg = WsMessage::SystemStatus { session_id, status };
            send_message(sender, &msg).await;
        }
        Err(err) => {
            send_error(sender, Some(&session_id), "status_failed", &err.to_string()).await;
        }
    }
}

async fn handle_calibrate(state: &Arc<AppState>, sender: &WsSender, session_id: String) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    if let Err(err) = session.dice.calibrate().await {
        send_error(sender, Some(&session_id), "calibrate_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: "Calibration complete".into(),
    };
    send_message(sender, &msg).await;
}

/// Enable or disable single tap interrupt notifications.
async fn handle_set_tap_interrupt(state: &Arc<AppState>, sender: &WsSender, session_id: String, enable: bool) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    let result = if enable {
        session.dice.enable_tap().await
    } else {
        session.dice.disable_tap().await
    };

    if let Err(err) = result {
        send_error(sender, Some(&session_id), "tap_interrupt_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: format!("Tap notifications {}", if enable { "enabled" } else { "disabled" }),
    };
    send_message(sender, &msg).await;
}

/// Enable or disable double tap interrupt notifications.
async fn handle_set_double_tap_interrupt(state: &Arc<AppState>, sender: &WsSender, session_id: String, enable: bool) {
    let sessions = state.sessions.lock().await;
    let session = match sessions.get(&session_id) {
        Some(s) => s,
        None => {
            send_error(sender, None, "session_not_found", &format!("session not found: {session_id}")).await;
            return;
        }
    };

    let result = if enable {
        session.dice.enable_double_tap().await
    } else {
        session.dice.disable_double_tap().await
    };

    if let Err(err) = result {
        send_error(sender, Some(&session_id), "double_tap_interrupt_failed", &err.to_string()).await;
        return;
    }

    let msg = WsMessage::Success {
        session_id,
        message: format!("Double tap notifications {}", if enable { "enabled" } else { "disabled" }),
    };
    send_message(sender, &msg).await;
}

/// Send a `WsMessage` as JSON to the WebSocket client.
///
/// Returns `true` if the message was sent successfully, `false` on serialization failure.
async fn send_message(sender: &WsSender, message: &WsMessage) -> bool {
    match serde_json::to_string(message) {
        Ok(json) => {
            let _ = sender.lock().await.send(Message::Text(json.into())).await;
            true
        }
        Err(err) => {
            error!(error = %err, "failed to serialize WebSocket message");
            false
        }
    }
}

/// Send an error message to the WebSocket client.
async fn send_error(sender: &WsSender, session_id: Option<&str>, code: &str, message: &str) {
    let msg = WsMessage::Error {
        session_id: session_id.map(|s| s.to_string()),
        code: code.to_string(),
        message: message.to_string(),
    };
    send_message(sender, &msg).await;
}
