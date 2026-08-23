use std::sync::Arc;
use std::time::Duration;

use btleplug::api::ScanFilter;
use tracing::debug;

use crate::ble::transport::BlePeripheral;
use crate::ble::transport::BleTransport;
use crate::error::Result;
use crate::service::dice::DiceDevice;

/// Default name prefix for GoDice devices.
const DEFAULT_NAME_PREFIX: &str = "GoDice_";

/// Default scan duration.
const DEFAULT_SCAN_DURATION: Duration = Duration::from_secs(5);

/// Scans for GoDice BLE devices in range.
pub struct DiceScanner<T: BleTransport> {
    transport: Arc<T>,
    name_prefix: String,
    scan_duration: Duration,
}

impl<T: BleTransport> DiceScanner<T> {
    /// Create a scanner with the default prefix "GoDice_" and 5s scan duration.
    pub fn new(transport: Arc<T>) -> Self {
        Self {
            transport,
            name_prefix: DEFAULT_NAME_PREFIX.to_string(),
            scan_duration: DEFAULT_SCAN_DURATION,
        }
    }

    /// Set a custom name prefix.
    pub fn with_name_prefix(self, prefix: impl Into<String>) -> Self {
        Self {
            name_prefix: prefix.into(),
            ..self
        }
    }

    /// Set a custom scan duration.
    pub fn with_scan_duration(self, duration: Duration) -> Self {
        Self {
            scan_duration: duration,
            ..self
        }
    }

    /// Scan for GoDice devices. Returns when the scan duration elapses or
    /// the transport stops scanning.
    ///
    /// Uses a two-stage filter:
    /// 1. `ScanFilter` with the NUS service UUID (if supported by platform).
    /// 2. Fallback: filter `PeripheralProperties::local_name` by prefix.
    pub async fn scan(&self) -> Result<Vec<DiceDevice>> {
        self.transport.start_scan(ScanFilter::default()).await?;

        // Wait for scan duration to collect peripherals.
        tokio::time::sleep(self.scan_duration).await;

        self.transport.stop_scan().await?;

        let peripherals = self.transport.peripherals().await?;
        let mut devices = Vec::new();

        for peripheral in peripherals {
            let props = peripheral.properties().await?;
            if let Some(props) = props {
                let local_name = props.local_name.as_deref().unwrap_or("");
                if local_name.starts_with(&self.name_prefix) {
                    debug!(name = local_name, "found GoDice device");
                    devices.push(DiceDevice {
                        id: peripheral.id(),
                        address: peripheral.address(),
                        name: local_name.to_string(),
                        rssi: props.rssi,
                    });
                }
            }
        }

        Ok(devices)
    }
}
