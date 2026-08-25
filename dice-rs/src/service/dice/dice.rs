use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use std::time::Duration;

use btleplug::api::{Characteristic, WriteType};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tracing::debug;
use tracing::trace;

use crate::ble::command::Command;
use crate::ble::ble_error::BleError;
use crate::ble::event::Event;
use crate::ble::transport::BlePeripheral;
use crate::ble::transport::BtleplugPeripheralWrapper;
use crate::error::DiceError;
use crate::error::Result;
use crate::model::acceleration::AccelerationOffset;
use crate::model::battery_level::BatteryLevel;
use crate::model::charging_state::ChargingState;
use crate::model::dice::DiceColor;
use crate::model::dice::DiceType;
use crate::model::led::LedColor;
use crate::model::led::PulseBlinkMode;
use crate::model::led::PulseLeds;
use crate::model::system_status::SystemStatus;
use crate::service::dice::event::DiceEvent;
use crate::service::dice::inner::DiceInner;
use crate::service::led_throttle_state::LedThrottleState;

/// Minimum interval between consecutive LED writes (milliseconds).
/// Rapid calls within this window are coalesced into a single write.
const LED_DEBOUNCE_MS: u64 = 30;

/// Timeout for request-response BLE queries (battery, color, calibration).
/// If the dice does not respond within this window, the caller receives
/// `DiceError::ResponseTimeout` and the pending sender is dropped, which
/// causes `is_canceled()` to return true so the notification task can
/// purge it from the FIFO queue before matching the next response.
const RESPONSE_TIMEOUT_SECS: u64 = 5;

/// Interval for the periodic connection health check.
const CONNECTION_MONITOR_INTERVAL_SECS: u64 = 5;

/// Handle to a connected GoDice device.
///
/// `Clone` is cheap (just an `Arc` clone). All clones share the same
/// underlying connection state.
#[derive(Clone)]
pub struct Dice {
    inner: Arc<DiceInner>,
}

impl Dice {
    /// Create a new `Dice` handle from a connected peripheral and discovered characteristics.
    pub(crate) fn new(peripheral: BtleplugPeripheralWrapper, name: String, write_char: Characteristic, notify_char: Characteristic) -> Self {
        let (event_sender, _) = broadcast::channel(64);
        let inner = Arc::new(DiceInner {
            name,
            peripheral,
            write_char,
            notify_char,
            event_sender,
            dice_type: Arc::new(AtomicU8::new(DiceType::D6.into())),
            pending_battery: Arc::new(Mutex::new(VecDeque::new())),
            pending_color: Arc::new(Mutex::new(VecDeque::new())),
            pending_calibration: Arc::new(Mutex::new(VecDeque::new())),
            notification_handle: Mutex::new(None),
            monitor_handle: Mutex::new(None),
            led_throttle: Mutex::new(LedThrottleState {
                pending: None,
                last_update: None,
            }),
            led_debounce_handle: Mutex::new(None),
            led_notify: Arc::new(tokio::sync::Notify::new()),
            calibration_offset: Arc::new(std::sync::RwLock::new(None)),
            charging_state: Arc::new(AtomicU8::new(ChargingState::NotCharging as u8)),
        });
        Self { inner }
    }

    /// Set both RGB LEDs to the given colors.
    ///
    /// Rapid successive calls are coalesced: if `set_leds` is called
    /// again within `LED_DEBOUNCE_MS`, only the most recent colors are
    /// written. This prevents BlueZ/DBus socket buffer overflow when an
    /// application fires many color changes in quick succession.
    pub async fn set_leds(&self, led1: LedColor, led2: LedColor) -> Result<()> {
        {
            let mut throttle = self.inner.led_throttle.lock().map_err(|_| DiceError::LockPoisoned)?;
            throttle.pending = Some((led1, led2));
            throttle.last_update = Some(tokio::time::Instant::now());
        }
        self.inner.led_notify.notify_one();
        Ok(())
    }

    /// Set both LEDs without debounce — writes immediately.
    ///
    /// Use this for one-shot LED commands where coalescing is undesirable
    /// (e.g. CLI commands, calibration sequences).
    pub async fn set_leds_immediate(&self, led1: LedColor, led2: LedColor) -> Result<()> {
        let data: Vec<u8> = Command::SetLeds { led1, led2 }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await
    }

    /// Flush a pending LED write immediately, bypassing the debounce.
    ///
    /// Called by the debounce background task after the quiet window
    /// has elapsed.
    async fn flush_led(&self) -> Result<()> {
        let (led1, led2) = {
            let mut throttle = self.inner.led_throttle.lock().map_err(|_| DiceError::LockPoisoned)?;
            match throttle.pending.take() {
                Some(colors) => colors,
                None => return Ok(()),
            }
        };
        let data: Vec<u8> = Command::SetLeds { led1, led2 }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await
    }

    /// Set both LEDs to the same color.
    pub async fn set_led(&self, color: LedColor) -> Result<()> {
        self.set_leds(color, color).await
    }

    /// Turn both LEDs off.
    pub async fn turn_off_leds(&self) -> Result<()> {
        self.set_leds(LedColor::OFF, LedColor::OFF).await
    }

    /// Pulse both LEDs with a color.
    ///
    /// `blink_mode` controls the blink pattern (rainbow or solid color).
    /// `leds` controls which LEDs participate in the pulse.
    pub async fn pulse_leds(
        &self,
        pulse_count: u8,
        on_time: u8,
        off_time: u8,
        color: LedColor,
        blink_mode: PulseBlinkMode,
        leds: PulseLeds,
    ) -> Result<()> {
        let data: Vec<u8> = Command::PulseLeds {
            pulse_count,
            on_time,
            off_time,
            color,
            blink_mode,
            leds,
        }
        .into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await
    }

    /// Pulse both LEDs with a color using a single pulse.
    ///
    /// Convenience method equivalent to `pulse_leds(1, on_time, off_time, color, PulseBlinkMode::Color, PulseLeds::Both)`.
    pub async fn pulse_once(&self, on_time: u8, off_time: u8, color: LedColor) -> Result<()> {
        self.pulse_leds(1, on_time, off_time, color, PulseBlinkMode::Color, PulseLeds::Both).await
    }

    /// Enable single tap interrupt notifications from the dice.
    ///
    /// After enabling, the dice will send `DiceEvent::Tap` when it detects a tap.
    /// Disabled by default — must be explicitly enabled.
    pub async fn enable_tap(&self) -> Result<()> {
        debug!(device = %self.inner.name, "sending SetTapInterrupt(true)");
        let data: Vec<u8> = Command::SetTapInterrupt { enabled: true }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithResponse).await?;
        debug!(device = %self.inner.name, "SetTapInterrupt(true) sent successfully");
        Ok(())
    }

    /// Disable single tap interrupt notifications.
    pub async fn disable_tap(&self) -> Result<()> {
        debug!(device = %self.inner.name, "sending SetTapInterrupt(false)");
        let data: Vec<u8> = Command::SetTapInterrupt { enabled: false }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithResponse).await?;
        debug!(device = %self.inner.name, "SetTapInterrupt(false) sent successfully");
        Ok(())
    }

    /// Enable double tap interrupt notifications from the dice.
    ///
    /// After enabling, the dice will send `DiceEvent::DoubleTap` when it detects a double tap.
    /// Disabled by default — must be explicitly enabled.
    pub async fn enable_double_tap(&self) -> Result<()> {
        debug!(device = %self.inner.name, "sending SetDoubleTapInterrupt(true)");
        let data: Vec<u8> = Command::SetDoubleTapInterrupt { enabled: true }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithResponse).await?;
        debug!(device = %self.inner.name, "SetDoubleTapInterrupt(true) sent successfully");
        Ok(())
    }

    /// Disable double tap interrupt notifications.
    pub async fn disable_double_tap(&self) -> Result<()> {
        debug!(device = %self.inner.name, "sending SetDoubleTapInterrupt(false)");
        let data: Vec<u8> = Command::SetDoubleTapInterrupt { enabled: false }.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithResponse).await?;
        debug!(device = %self.inner.name, "SetDoubleTapInterrupt(false) sent successfully");
        Ok(())
    }

    /// Send initialization command to the dice.
    ///
    /// Sets roll detection sensitivity and LED configuration.
    /// Matches the Unity demo's `SendInitializationMessage` which is sent
    /// immediately after connection. The sensitivity value affects tap
    /// detection thresholds.
    pub async fn init(&self) -> Result<()> {
        let data: Vec<u8> = Command::Init {
            sensitivity: 30,
            pulse_count: 3,
            on_time: 50,
            off_time: 50,
            color: LedColor::GREEN,
            blink_mode: PulseBlinkMode::Color,
            leds: PulseLeds::Both,
        }
        .into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithResponse).await
    }

    /// Request battery level (0–100 percent).
    pub async fn get_battery_level(&self) -> Result<BatteryLevel> {
        let (tx, rx) = oneshot::channel();
        self.inner.pending_battery.lock().map_err(|_| DiceError::LockPoisoned)?.push_back(tx);
        let data: Vec<u8> = Command::GetBatteryLevel.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await?;
        let timeout = Duration::from_secs(RESPONSE_TIMEOUT_SECS);
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(level)) => Ok(BatteryLevel::from(level)),
            Ok(Err(_)) => Err(DiceError::ResponseTimeout(timeout)),
            Err(_) => Err(DiceError::ResponseTimeout(timeout)),
        }
    }

    /// Request dice color.
    pub async fn get_color(&self) -> Result<DiceColor> {
        let (tx, rx) = oneshot::channel();
        self.inner.pending_color.lock().map_err(|_| DiceError::LockPoisoned)?.push_back(tx);
        let data: Vec<u8> = Command::GetDiceColor.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await?;
        let timeout = Duration::from_secs(RESPONSE_TIMEOUT_SECS);
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(color)) => Ok(color),
            Ok(Err(_)) => Err(DiceError::ResponseTimeout(timeout)),
            Err(_) => Err(DiceError::ResponseTimeout(timeout)),
        }
    }

    /// Subscribe to dice events. Each subscriber gets its own receiver.
    /// If the broadcaster's buffer is full, slow subscribers may miss events.
    pub fn subscribe(&self) -> broadcast::Receiver<DiceEvent> {
        self.inner.event_sender.subscribe()
    }

    /// Set the dice type for face value interpretation.
    /// This is a client-side setting; no BLE command is sent.
    /// Synchronous — uses `AtomicU8::store` instead of an async lock.
    pub fn set_dice_type(&self, dice_type: DiceType) {
        self.inner.dice_type.store(dice_type.into(), Ordering::Relaxed);
    }

    /// Check if the dice is currently connected.
    pub async fn is_connected(&self) -> Result<bool> {
        self.inner.peripheral.is_connected().await
    }

    /// Returns the advertised device name (e.g. "GoDice_7D8E7D_O_v04").
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Check if the dice is currently charging.
    ///
    /// Returns the last known charging state from the notification task.
    /// Returns `NotCharging` until a charging status notification has been received.
    pub fn charging_state(&self) -> ChargingState {
        match self.inner.charging_state.load(Ordering::Relaxed) {
            1 => ChargingState::Charging,
            _ => ChargingState::NotCharging,
        }
    }

    /// Query RSSI (signal strength) in dBm.
    ///
    /// Tries `read_rssi()` first (reads from BlueZ device properties on Linux),
    /// falls back to cached `properties().rssi` if that fails.
    pub async fn rssi(&self) -> Result<Option<i16>> {
        match self.inner.peripheral.read_rssi().await {
            Ok(rssi) => Ok(Some(rssi)),
            Err(_) => {
                let props = self.inner.peripheral.properties().await?;
                Ok(props.and_then(|p| p.rssi))
            }
        }
    }

    /// Get comprehensive system status in a single call.
    /// Performs battery level and color queries concurrently.
    pub async fn system_status(&self) -> Result<SystemStatus> {
        let (battery, color) = tokio::try_join!(self.get_battery_level(), self.get_color(),)?;
        let connected = self.is_connected().await?;
        let rssi = self.rssi().await?;
        Ok(SystemStatus::builder()
            .battery_level(battery)
            .color(color)
            .connected(connected)
            .rssi(rssi)
            .build())
    }

    /// Disconnect from the dice.
    ///
    /// Aborts the notification and connection monitor tasks, then
    /// calls `peripheral.disconnect()`.
    pub async fn disconnect(&self) -> Result<()> {
        self.abort_tasks();
        self.inner.peripheral.disconnect().await
    }

    /// Abort the notification and connection monitor tasks.
    fn abort_tasks(&self) {
        if let Ok(mut handle) = self.inner.notification_handle.lock()
            && let Some(task) = handle.take()
        {
            task.abort();
        }
        if let Ok(mut handle) = self.inner.monitor_handle.lock()
            && let Some(task) = handle.take()
        {
            task.abort();
        }
        if let Ok(mut handle) = self.inner.led_debounce_handle.lock()
            && let Some(task) = handle.take()
        {
            task.abort();
        }
    }

    /// Internal reconnect: re-subscribe and re-spawn tasks.
    pub(crate) async fn reconnect_internal(&self) -> Result<()> {
        self.abort_tasks();
        self.inner.peripheral.subscribe(&self.inner.notify_char).await?;
        self.spawn_notification_task().await?;
        self.spawn_led_debounce_task();
        self.spawn_connection_monitor();
        Ok(())
    }

    /// Spawn the notification parsing task.
    pub(crate) async fn spawn_notification_task(&self) -> Result<()> {
        let notifications = self.inner.peripheral.notifications().await?;
        let dice_type = self.inner.dice_type.clone();
        let event_sender = self.inner.event_sender.clone();
        let pending_battery = self.inner.pending_battery.clone();
        let pending_color = self.inner.pending_color.clone();
        let pending_calibration = self.inner.pending_calibration.clone();
        let calibration_offset = self.inner.calibration_offset.clone();
        let charging_state = self.inner.charging_state.clone();

        let handle = tokio::spawn(async move {
            let mut notifications = notifications;
            while let Some(notification) = notifications.next().await {
                let data = &notification.value;
                let event = match Event::parse(data) {
                    Ok(event) => event,
                    Err(error) => {
                        debug!(?error, data = ?data, "failed to parse notification");
                        continue;
                    }
                };

                trace!(?event, "received BLE notification");

                match event {
                    Event::RollStart => {
                        let _ = event_sender.send(DiceEvent::RollStart);
                    }
                    Event::Stable { acceleration } => {
                        let dice_type = DiceType::try_from(dice_type.load(Ordering::Relaxed)).unwrap_or(DiceType::D6);
                        let offset = calibration_offset.read().map(|guard| *guard).unwrap_or(None);
                        match acceleration.interpret_to_face(dice_type, offset) {
                            Ok(face) => {
                                let _ = event_sender.send(DiceEvent::Stable { face, acceleration });
                            }
                            Err(error) => debug!(?error, "failed to interpret face value"),
                        }
                    }
                    Event::TiltStable { acceleration } => {
                        let dice_type = DiceType::try_from(dice_type.load(Ordering::Relaxed)).unwrap_or(DiceType::D6);
                        let offset = calibration_offset.read().map(|guard| *guard).unwrap_or(None);
                        match acceleration.interpret_to_face(dice_type, offset) {
                            Ok(face) => {
                                let _ = event_sender.send(DiceEvent::TiltStable { face, acceleration });
                            }
                            Err(error) => debug!(?error, "failed to interpret face value"),
                        }
                    }
                    Event::FakeStable { acceleration } => {
                        let dice_type = DiceType::try_from(dice_type.load(Ordering::Relaxed)).unwrap_or(DiceType::D6);
                        let offset = calibration_offset.read().map(|guard| *guard).unwrap_or(None);
                        match acceleration.interpret_to_face(dice_type, offset) {
                            Ok(face) => {
                                let _ = event_sender.send(DiceEvent::FakeStable { face, acceleration });
                            }
                            Err(error) => debug!(?error, "failed to interpret face value"),
                        }
                    }
                    Event::MoveStable { acceleration } => {
                        let dice_type = DiceType::try_from(dice_type.load(Ordering::Relaxed)).unwrap_or(DiceType::D6);
                        let offset = calibration_offset.read().map(|guard| *guard).unwrap_or(None);
                        match acceleration.interpret_to_face(dice_type, offset) {
                            Ok(face) => {
                                let _ = event_sender.send(DiceEvent::MoveStable { face, acceleration });
                            }
                            Err(error) => debug!(?error, "failed to interpret face value"),
                        }
                    }
                    Event::BatteryLevel { level } => {
                        if let Ok(mut queue) = pending_battery.lock() {
                            queue.retain(|s| !s.is_closed());
                            if let Some(sender) = queue.pop_front()
                                && sender.send(level).is_err()
                            {
                                debug!("battery level response dropped: receiver gone");
                            }
                        }
                    }
                    Event::DiceColor { color } => {
                        if let Ok(mut queue) = pending_color.lock() {
                            queue.retain(|s| !s.is_closed());
                            if let Some(sender) = queue.pop_front()
                                && sender.send(color).is_err()
                            {
                                debug!("dice color response dropped: receiver gone");
                            }
                        }
                    }
                    Event::Calibrated { success } => {
                        if let Ok(mut queue) = pending_calibration.lock() {
                            queue.retain(|s| !s.is_closed());
                            if let Some(sender) = queue.pop_front()
                                && sender.send(success).is_err()
                            {
                                debug!("calibration response dropped: receiver gone");
                            }
                        }
                    }
                    Event::Charging { charging } => {
                        let state = ChargingState::from(charging);
                        charging_state.store(state as u8, Ordering::Relaxed);
                        if event_sender.send(DiceEvent::Charging { state }).is_err() {
                            debug!("charging event dropped: no subscribers");
                        }
                    }
                    Event::Tap => {
                        if event_sender.send(DiceEvent::Tap).is_err() {
                            debug!("tap event dropped: no subscribers");
                        }
                    }
                    Event::DoubleTap => {
                        if event_sender.send(DiceEvent::DoubleTap).is_err() {
                            debug!("double-tap event dropped: no subscribers");
                        }
                    }
                }
            }
            let _ = event_sender.send(DiceEvent::Disconnected);
        });

        if let Ok(mut guard) = self.inner.notification_handle.lock() {
            *guard = Some(handle);
        }
        Ok(())
    }

    /// Spawn the LED debounce background task.
    ///
    /// Waits for a quiet window of `LED_DEBOUNCE_MS` with no new `set_leds`
    /// calls, then flushes the most recent pending color to the BLE transport.
    pub(crate) fn spawn_led_debounce_task(&self) {
        let dice = self.clone();
        let handle = tokio::spawn(async move {
            let debounce = Duration::from_millis(LED_DEBOUNCE_MS);
            loop {
                let has_pending = {
                    let throttle = dice.inner.led_throttle.lock();
                    match throttle {
                        Ok(throttle) => throttle.pending.is_some(),
                        Err(_) => break,
                    }
                };

                if !has_pending {
                    dice.inner.led_notify.notified().await;
                }

                tokio::time::sleep(debounce).await;

                if let Err(error) = dice.flush_led().await {
                    debug!(error = %error, "failed to flush debounced LED write");
                }
            }
        });

        if let Ok(mut guard) = self.inner.led_debounce_handle.lock() {
            *guard = Some(handle);
        }
    }

    /// Spawn a background task that periodically checks connection state
    /// and emits `DiceEvent::Disconnected` if the BLE link is lost.
    pub(crate) fn spawn_connection_monitor(&self) {
        let dice = self.clone();
        let event_sender = self.inner.event_sender.clone();
        let interval = Duration::from_secs(CONNECTION_MONITOR_INTERVAL_SECS);
        let handle = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                match dice.is_connected().await {
                    Ok(true) => {}
                    Ok(false) | Err(_) => {
                        if event_sender.send(DiceEvent::Disconnected).is_err() {
                            debug!("no subscribers for Disconnected event");
                        }
                        break;
                    }
                }
            }
        });

        if let Ok(mut guard) = self.inner.monitor_handle.lock() {
            *guard = Some(handle);
        }
    }

    /// Hardware calibration via BLE (tentative — opcode unconfirmed).
    pub async fn calibrate(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.inner.pending_calibration.lock().map_err(|_| DiceError::LockPoisoned)?.push_back(tx);
        let data: Vec<u8> = Command::Calibrate.into();
        self.inner.peripheral.write(&self.inner.write_char, &data, WriteType::WithoutResponse).await?;
        let timeout = Duration::from_secs(RESPONSE_TIMEOUT_SECS);
        let success = match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(success)) => success,
            Ok(Err(_)) => return Err(DiceError::ResponseTimeout(timeout)),
            Err(_) => return Err(DiceError::ResponseTimeout(timeout)),
        };
        if success { Ok(()) } else { Err(DiceError::CalibrationFailed) }
    }

    /// Software calibration: capture next Stable event and compute offset.
    pub async fn calibrate_software(&self) -> Result<AccelerationOffset> {
        let mut receiver = self.subscribe();
        loop {
            match receiver.recv().await {
                Ok(DiceEvent::Stable { acceleration, .. }) => {
                    let dice_type = DiceType::try_from(self.inner.dice_type.load(Ordering::Relaxed)).unwrap_or(DiceType::D6);
                    let offset = acceleration.offset_to(dice_type);
                    *self.inner.calibration_offset.write().map_err(|_| DiceError::LockPoisoned)? = Some(offset);
                    return Ok(offset);
                }
                Ok(_) => continue,
                Err(broadcast::error::RecvError::Closed) => {
                    return Err(BleError::ConnectionLost.into());
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    }

    /// Clear software calibration offset.
    pub fn clear_software_calibration(&self) -> Result<()> {
        *self.inner.calibration_offset.write().map_err(|_| DiceError::LockPoisoned)? = None;
        Ok(())
    }
}
