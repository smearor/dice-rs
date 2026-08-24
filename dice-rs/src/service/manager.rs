use std::sync::Arc;
use std::time::Duration;

use tracing::debug;

use crate::ble::transport::BlePeripheral;
use crate::ble::transport::BleTransport;
use crate::ble::transport::BtleplugTransport;
use crate::ble::uuids::NUS_NOTIFY_CHAR_UUID;
use crate::ble::uuids::NUS_WRITE_CHAR_UUID;
use crate::error::DiceError;
use crate::error::Result;
use crate::service::dice::Dice;
use crate::service::dice::DiceDevice;
use crate::service::scanner::DiceScanner;

/// Manages BLE adapter and multiple dice connections.
pub struct DiceManager {
    transport: Arc<BtleplugTransport>,
}

impl DiceManager {
    /// Create a new manager. Internally calls `Manager::new()` and selects
    /// the first available Bluetooth adapter.
    pub async fn new() -> Result<Self> {
        let transport = BtleplugTransport::new().await?;
        Ok(Self {
            transport: Arc::new(transport),
        })
    }

    /// Create a scanner for discovering GoDice devices.
    pub fn scanner(&self) -> DiceScanner<BtleplugTransport> {
        DiceScanner::new(self.transport.clone())
    }

    /// Scan for GoDice devices using the default scanner settings.
    pub async fn scan(&self) -> Result<Vec<DiceDevice>> {
        self.scanner().scan().await
    }

    /// Connect to a discovered device.
    ///
    /// Performs:
    /// 1. `peripheral.connect()` (with up to 3 retries and 1s backoff)
    /// 2. `peripheral.discover_services()`
    /// 3. Find write char (`6e400002`) and notify char (`6e400003`)
    /// 4. `peripheral.subscribe(notify_char)`
    /// 5. `peripheral.notifications()` → spawn parse task
    /// 6. Return `Dice` handle
    ///
    /// The retry loop handles transient connection failures (e.g. a GoDice
    /// that is advertising but not yet accepting connections while charging
    /// from 0% battery).
    pub async fn connect(&self, device: &DiceDevice) -> Result<Dice> {
        let peripherals = self.transport.peripherals().await?;
        let peripheral = peripherals
            .into_iter()
            .find(|p| p.id() == device.id)
            .ok_or_else(|| DiceError::ConnectionFailed(format!("peripheral not found for {}", device.name)))?;

        let max_retries = 3;
        let backoff = Duration::from_secs(1);
        let mut last_error = DiceError::ConnectionFailed("no attempt made".to_string());
        for attempt in 0..max_retries {
            match peripheral.connect().await {
                Ok(()) => break,
                Err(error) => {
                    last_error = error;
                    debug!(attempt, %last_error, device = %device.name, "connect attempt failed, retrying");
                    if attempt + 1 < max_retries {
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }
        if !peripheral.is_connected().await? {
            return Err(last_error);
        }

        peripheral.discover_services().await?;

        let write_char = peripheral
            .characteristic(NUS_WRITE_CHAR_UUID)
            .ok_or_else(|| DiceError::CharacteristicNotFound("NUS write".to_string()))?;
        let notify_char = peripheral
            .characteristic(NUS_NOTIFY_CHAR_UUID)
            .ok_or_else(|| DiceError::CharacteristicNotFound("NUS notify".to_string()))?;

        peripheral.subscribe(&notify_char).await?;

        let dice = Dice::new(peripheral, device.name.clone(), write_char, notify_char);
        dice.spawn_notification_task().await?;
        dice.spawn_led_debounce_task();
        dice.spawn_connection_monitor();

        Ok(dice)
    }

    /// Attempt to reconnect to a disconnected dice.
    ///
    /// Retries with exponential backoff until success or max retries.
    /// Mirrors the JS API's `attemptReconnect` behavior.
    pub async fn reconnect(&self, dice: &Dice) -> Result<()> {
        let mut backoff = Duration::from_millis(500);
        let max_backoff = Duration::from_secs(5);
        let max_retries = 10;

        for attempt in 0..max_retries {
            if dice.is_connected().await? {
                return Ok(());
            }
            if let Err(error) = dice.reconnect_internal().await {
                debug!(attempt, %error, "reconnect attempt failed");
            }
            tokio::time::sleep(backoff).await;
            backoff = (backoff * 2).min(max_backoff);
        }
        Err(DiceError::ReconnectFailed)
    }

    /// Disconnect all dice and release the BLE adapter.
    pub async fn shutdown(&self) -> Result<()> {
        self.transport.stop_scan().await
    }
}
