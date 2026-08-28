use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;

use dice_rs::service::dice::Dice;
use dice_rs::service::manager::DiceManager;
use tracing::debug;

/// Interval for periodic auto-scan after the startup burst (seconds).
const AUTO_SCAN_INTERVAL_SECS: u64 = 15;

/// Fast auto-scan interval during startup burst (seconds).
const AUTO_SCAN_FAST_INTERVAL_SECS: u64 = 5;

/// Duration of the startup burst period (seconds).
const AUTO_SCAN_BURST_DURATION_SECS: u64 = 60;

/// Events emitted by `ConnectionService` for the UI to react to.
pub enum ConnectionEvent {
    /// A scan was started.
    ScanStarted,
    /// The scan completed but no devices were found.
    NoDevicesFound,
    /// The scan found devices; `count` is the total number.
    DevicesFound(usize),
    /// A dice was successfully connected during a manual scan.
    DiceConnected { dice: Dice, manager: Arc<DiceManager> },
    /// A dice connection failed during a manual scan.
    DiceConnectionFailed { name: String, error: String },
    /// A scan operation failed.
    ScanFailed(String),
    /// A new dice was connected by the auto-scan background loop.
    AutoScanDiceConnected { dice: Dice, manager: Arc<DiceManager> },
}

/// Service for dice discovery and connection management.
///
/// Encapsulates scanning, connecting, and auto-scan logic, keeping
/// `MainWindow` free of business logic. Events are emitted via a
/// `std::sync::mpsc` channel that the UI polls on the GTK main thread.
#[derive(Clone)]
pub struct ConnectionService {
    manager: Arc<DiceManager>,
    connected_ids: Arc<Mutex<HashSet<String>>>,
}

impl ConnectionService {
    /// Create a new connection service.
    pub fn new(manager: Arc<DiceManager>) -> Self {
        Self {
            manager,
            connected_ids: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Returns a reference to the underlying dice manager.
    pub fn manager(&self) -> &Arc<DiceManager> {
        &self.manager
    }

    /// Reset tracked connections (e.g. before a fresh manual scan).
    pub fn clear_connected(&self) {
        if let Ok(mut ids) = self.connected_ids.lock() {
            ids.clear();
        }
    }

    /// Start a manual scan: discover devices and connect to all of them.
    ///
    /// Emits `ConnectionEvent`s via `sender` as scan and connection
    /// operations progress. This method spawns a tokio task and returns
    /// immediately.
    pub fn scan_once(&self, sender: mpsc::Sender<ConnectionEvent>) {
        let manager = self.manager.clone();
        let connected_ids = self.connected_ids.clone();

        tokio::spawn(async move {
            let _ = sender.send(ConnectionEvent::ScanStarted);

            let scan_result = manager.scan().await;
            match scan_result {
                Ok(devices) => {
                    if devices.is_empty() {
                        let _ = sender.send(ConnectionEvent::NoDevicesFound);
                        return;
                    }
                    let count = devices.len();
                    let _ = sender.send(ConnectionEvent::DevicesFound(count));

                    for device in devices {
                        let device_id = format!("{:?}", device.id);
                        if let Ok(mut ids) = connected_ids.lock() {
                            ids.insert(device_id);
                        }

                        let device_name = device.name.clone();
                        let connect_manager = manager.clone();
                        match connect_manager.connect(&device).await {
                            Ok(dice) => {
                                let _ = sender.send(ConnectionEvent::DiceConnected {
                                    dice,
                                    manager: connect_manager.clone(),
                                });
                            }
                            Err(error) => {
                                debug!(error = %error, device = %device_name, "connection failed");
                                let _ = sender.send(ConnectionEvent::DiceConnectionFailed {
                                    name: device_name,
                                    error: error.to_string(),
                                });
                            }
                        }
                    }
                }
                Err(error) => {
                    debug!(error = %error, "scan failed");
                    let _ = sender.send(ConnectionEvent::ScanFailed(error.to_string()));
                }
            }
        });
    }

    /// Start the background auto-scan loop.
    ///
    /// Discovers and connects to new dice automatically. Existing
    /// connections are preserved — only newly discovered devices are
    /// connected. Emits `AutoScanDiceConnected` via `sender` when a
    /// new dice is connected.
    pub fn start_auto_scan(&self, sender: mpsc::Sender<ConnectionEvent>) {
        let manager = self.manager.clone();
        let connected_ids = self.connected_ids.clone();

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            loop {
                let scan_result = manager.scan().await;
                match scan_result {
                    Ok(devices) => {
                        for device in devices {
                            let device_id = format!("{:?}", device.id);
                            let already_connected = {
                                let mut ids = match connected_ids.lock() {
                                    Ok(guard) => guard,
                                    Err(error) => {
                                        debug!(error = %error, "connected_ids mutex poisoned");
                                        continue;
                                    }
                                };
                                if ids.contains(&device_id) {
                                    true
                                } else {
                                    ids.insert(device_id.clone());
                                    false
                                }
                            };
                            if already_connected {
                                continue;
                            }

                            let device_name = device.name.clone();
                            debug!(device = %device_name, "auto-scan: connecting new device");
                            let connect_manager = manager.clone();
                            let connect_sender = sender.clone();
                            let connect_ids = connected_ids.clone();
                            tokio::spawn(async move {
                                match connect_manager.connect(&device).await {
                                    Ok(dice) => {
                                        let _ = connect_sender.send(ConnectionEvent::AutoScanDiceConnected {
                                            dice,
                                            manager: connect_manager.clone(),
                                        });
                                    }
                                    Err(error) => {
                                        debug!(error = %error, device = %device_name, "auto-scan: connection failed");
                                        if let Ok(mut ids) = connect_ids.lock() {
                                            ids.remove(&device_id);
                                        }
                                    }
                                }
                            });
                        }
                    }
                    Err(error) => {
                        debug!(error = %error, "auto-scan failed");
                    }
                }

                let elapsed = start.elapsed().as_secs();
                let interval = if elapsed < AUTO_SCAN_BURST_DURATION_SECS {
                    AUTO_SCAN_FAST_INTERVAL_SECS
                } else {
                    AUTO_SCAN_INTERVAL_SECS
                };
                tokio::time::sleep(Duration::from_secs(interval)).await;
            }
        });
    }
}
