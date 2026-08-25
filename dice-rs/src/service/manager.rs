use std::sync::Arc;
use std::time::Duration;

use tracing::debug;

use crate::ble::ble_error::BleError;
use crate::ble::nus_characteristic::NusCharacteristic;
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

    /// Find a discovered device by MAC address (partial match).
    ///
    /// Scans for devices and returns the first whose address contains the
    /// given substring. Useful for matching a short MAC prefix.
    pub async fn find_device_by_address(&self, address: &str) -> Result<DiceDevice> {
        let devices = self.scan().await?;
        devices
            .into_iter()
            .find(|d| d.address.to_string().contains(address))
            .ok_or_else(|| BleError::device_not_found(address).into())
    }

    /// Connect to a dice by MAC address (scans first).
    ///
    /// Convenience method combining `find_device_by_address` and `connect`.
    pub async fn connect_by_address(&self, address: &str) -> Result<Dice> {
        let device = self.find_device_by_address(address).await?;
        self.connect(&device).await
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
            .ok_or_else(|| DiceError::from(BleError::peripheral_not_found(&device.name)))?;

        let max_retries = 3;
        let backoff = Duration::from_secs(1);
        let mut last_error: DiceError = BleError::NoAttemptMade.into();
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
            .ok_or_else(|| DiceError::from(BleError::characteristic_not_found(NusCharacteristic::Write)))?;
        let notify_char = peripheral
            .characteristic(NUS_NOTIFY_CHAR_UUID)
            .ok_or_else(|| DiceError::from(BleError::characteristic_not_found(NusCharacteristic::Notify)))?;

        peripheral.subscribe(&notify_char).await?;

        let dice = Dice::new(peripheral, device.name.clone(), write_char, notify_char);
        dice.spawn_notification_task().await?;
        dice.spawn_led_debounce_task();
        dice.spawn_connection_monitor();

        dice.init().await?;

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
        Err(BleError::ReconnectFailed.into())
    }

    /// Disconnect all dice and release the BLE adapter.
    pub async fn shutdown(&self) -> Result<()> {
        self.transport.stop_scan().await
    }

    /// Disconnect a peripheral by MAC address.
    ///
    /// Scans for peripherals and disconnects the one matching the given address.
    /// Useful for cleaning up stale connections before scanning, since BlueZ
    /// does not deliver RSSI advertisement updates for connected devices.
    pub async fn disconnect_by_address(&self, address: &str) -> Result<()> {
        let peripherals = self.transport.peripherals().await?;
        for peripheral in peripherals {
            if peripheral.address().to_string().contains(address)
                && peripheral.is_connected().await?
            {
                debug!(address = %peripheral.address(), "disconnecting peripheral");
                peripheral.disconnect().await?;
            }
        }
        Ok(())
    }

    /// Disconnect all connected GoDice devices.
    ///
    /// Iterates all known peripherals, filters by the GoDice name prefix,
    /// and disconnects those that are currently connected. Returns the
    /// number of devices that were disconnected.
    pub async fn disconnect_all(&self) -> Result<usize> {
        let peripherals = self.transport.peripherals().await?;
        let mut count = 0;
        for peripheral in peripherals {
            let props = peripheral.properties().await?;
            if let Some(props) = props {
                let local_name = props.local_name.as_deref().unwrap_or("");
                if local_name.starts_with("GoDice_") && peripheral.is_connected().await? {
                    debug!(name = local_name, address = %peripheral.address(), "disconnecting GoDice");
                    peripheral.disconnect().await?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}
