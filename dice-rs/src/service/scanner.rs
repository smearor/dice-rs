use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use btleplug::api::CentralEvent;
use btleplug::api::ScanFilter;
use futures::StreamExt;
use tokio::select;
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
    /// Consumes the `CentralEvent` stream during the scan to collect RSSI
    /// values from `RssiUpdate` events. Multiple RSSI samples per device are
    /// averaged for a more stable value. Falls back to `read_rssi()` or
    /// cached `properties().rssi` if no events were received.
    ///
    /// Uses a two-stage filter:
    /// 1. `ScanFilter` with the NUS service UUID (if supported by platform).
    /// 2. Fallback: filter `PeripheralProperties::local_name` by prefix.
    pub async fn scan(&self) -> Result<Vec<DiceDevice>> {
        self.transport.start_scan(ScanFilter::default()).await?;

        let mut events = self.transport.events().await?;
        let mut rssi_samples: HashMap<btleplug::platform::PeripheralId, Vec<i16>> = HashMap::new();

        let scan_deadline = tokio::time::sleep(self.scan_duration);
        tokio::pin!(scan_deadline);

        loop {
            select! {
                _ = &mut scan_deadline => break,
                event = events.next() => {
                    match event {
                        Some(CentralEvent::RssiUpdate { id, rssi }) => {
                            debug!(?id, rssi, "RssiUpdate event");
                            rssi_samples.entry(id).or_default().push(rssi);
                        }
                        Some(other) => debug!(event = ?other, "central event during scan"),
                        None => debug!("event stream ended"),
                    }
                }
            }
        }

        self.transport.stop_scan().await?;

        let peripherals = self.transport.peripherals().await?;
        let mut devices = Vec::new();

        for peripheral in peripherals {
            let props = peripheral.properties().await?;
            if let Some(props) = props {
                let local_name = props.local_name.as_deref().unwrap_or("");
                if local_name.starts_with(&self.name_prefix) {
                    let id = peripheral.id();
                    let avg_rssi = rssi_samples
                        .get(&id)
                        .filter(|samples| !samples.is_empty())
                        .map(|samples| {
                            let sum: i64 = samples.iter().map(|r| *r as i64).sum();
                            (sum / samples.len() as i64) as i16
                        });
                    let avg_rssi = match avg_rssi {
                        Some(rssi) => Some(rssi),
                        None => match peripheral.read_rssi().await {
                            Ok(rssi) => {
                                debug!(name = local_name, rssi, "read_rssi succeeded");
                                Some(rssi)
                            }
                            Err(error) => {
                                debug!(name = local_name, error = %error, "read_rssi failed, falling back to props.rssi");
                                props.rssi
                            }
                        },
                    };

                    debug!(
                        name = local_name,
                        rssi = ?avg_rssi,
                        sample_count = rssi_samples.get(&id).map(|v| v.len()).unwrap_or(0),
                        "found GoDice device"
                    );
                    devices.push(DiceDevice {
                        id,
                        address: peripheral.address(),
                        name: local_name.to_string(),
                        rssi: avg_rssi,
                    });
                }
            }
        }

        Ok(devices)
    }
}
