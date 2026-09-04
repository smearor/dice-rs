use crate::error::Result;
use crate::error::YahtzeeError;
use crate::services::slot_mapping::SlotMapping;
use dice_rs::Dice;
use dice_rs::DiceColor;
use dice_rs::DiceManager;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tracing::debug;
use tracing::info;

/// Events emitted by `DiceService` for the UI to react to.
#[derive(Debug, Clone)]
pub enum DiceServiceEvent {
    /// A scan was started.
    ScanStarted,
    /// The scan completed but no devices were found.
    NoDevicesFound,
    /// The scan found devices; `count` is the total number.
    DevicesFound(usize),
    /// A specific device was found during scanning, with its name and inferred color.
    DeviceFound {
        /// The device name (e.g. "GoDice_320FF4_G_v04").
        name: String,
        /// The physical dice color inferred from the name.
        color: DiceColor,
    },
    /// A connection attempt is starting for a specific device.
    Connecting {
        /// The device name.
        name: String,
        /// The physical dice color inferred from the name.
        color: DiceColor,
    },
    /// A dice was successfully connected and assigned to a slot.
    DiceAssigned {
        /// The slot the dice was assigned to.
        slot: u8,
        /// The device name.
        name: String,
        /// The physical dice color.
        color: DiceColor,
    },
    /// A dice connection failed.
    DiceConnectionFailed {
        /// The device name.
        name: String,
        /// The error message.
        error: String,
    },
    /// A scan operation failed.
    ScanFailed(String),
    /// All 5 dice slots are now assigned.
    AllSlotsAssigned,
}

/// Service for dice discovery, connection, and slot mapping.
///
/// Wraps `DiceManager` to provide Yahtzee-specific dice management:
/// scanning for GoDice, connecting to exactly 5 dice, and mapping
/// them to game slots. Events are emitted via a `tokio::sync::broadcast`
/// channel for async consumers.
#[derive(Clone)]
pub struct DiceService {
    /// The underlying dice-rs manager.
    manager: Arc<DiceManager>,
    /// Slot mapping shared with LED service and event bridge.
    mapping: SlotMapping,
    /// Broadcast channel for service events.
    event_sender: tokio::sync::broadcast::Sender<DiceServiceEvent>,
    /// Track which device names have already been assigned.
    assigned_names: Arc<Mutex<Vec<String>>>,
    /// Device names that were swapped out and should not be reconnected.
    blacklisted_names: Arc<Mutex<Vec<String>>>,
    /// Flag to prevent concurrent scan loops.
    is_scanning: Arc<AtomicBool>,
}

impl DiceService {
    /// Create a new dice service.
    pub fn new(manager: Arc<DiceManager>, mapping: SlotMapping) -> Self {
        let (event_sender, _) = tokio::sync::broadcast::channel(64);
        Self {
            manager,
            mapping,
            event_sender,
            assigned_names: Arc::new(Mutex::new(Vec::new())),
            blacklisted_names: Arc::new(Mutex::new(Vec::new())),
            is_scanning: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Subscribe to service events.
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DiceServiceEvent> {
        self.event_sender.subscribe()
    }

    /// Get the slot mapping.
    pub fn mapping(&self) -> SlotMapping {
        self.mapping.clone()
    }

    /// Start scanning for GoDice devices and connecting them to slots.
    ///
    /// This method spawns a tokio task and returns immediately.
    /// The task retries scanning and connecting periodically until
    /// all 5 slots are assigned. Progress is reported via the event channel.
    /// Calling this while a scan loop is already running is a no-op.
    pub fn scan_and_connect(&self) {
        if self.is_scanning.swap(true, Ordering::SeqCst) {
            debug!("scan_and_connect already in progress, skipping");
            return;
        }

        let manager = self.manager.clone();
        let mapping = self.mapping.clone();
        let sender = self.event_sender.clone();
        let assigned_names = self.assigned_names.clone();
        let blacklisted_names = self.blacklisted_names.clone();
        let is_scanning = self.is_scanning.clone();

        tokio::spawn(async move {
            let retry_interval = Duration::from_secs(5);
            loop {
                if mapping.is_complete() {
                    break;
                }

                let _ = sender.send(DiceServiceEvent::ScanStarted);

                let scan_result = manager.scan().await;
                let devices = match scan_result {
                    Ok(devices) => {
                        if devices.is_empty() {
                            let _ = sender.send(DiceServiceEvent::NoDevicesFound);
                            tokio::time::sleep(retry_interval).await;
                            continue;
                        }
                        let count = devices.len();
                        let _ = sender.send(DiceServiceEvent::DevicesFound(count));
                        devices
                    }
                    Err(error) => {
                        debug!(error = %error, "scan failed");
                        let _ = sender.send(DiceServiceEvent::ScanFailed(error.to_string()));
                        tokio::time::sleep(retry_interval).await;
                        continue;
                    }
                };

                let device_names: Vec<String> = devices.iter().map(|d| d.name.clone()).collect();

                for device in devices {
                    if mapping.is_complete() {
                        break;
                    }

                    // Skip if already assigned or blacklisted (swapped out)
                    let skip = {
                        let names = match assigned_names.lock() {
                            Ok(n) => n,
                            Err(_) => continue,
                        };
                        names.contains(&device.name)
                    } || {
                        let blacklist = match blacklisted_names.lock() {
                            Ok(b) => b,
                            Err(_) => continue,
                        };
                        blacklist.contains(&device.name)
                    };
                    if skip {
                        continue;
                    }

                    // Emit DeviceFound so the UI can show the dice before connecting
                    let device_color = device.color().unwrap_or(dice_rs::DiceColor::Black);
                    let _ = sender.send(DiceServiceEvent::DeviceFound {
                        name: device.name.clone(),
                        color: device_color,
                    });

                    // Find the next empty slot
                    let slot_idx = {
                        let mapping_clone = mapping.clone();
                        let empty = (0..crate::models::dice_slot::DiceSlot::COUNT as u8).find(|&i| {
                            let slot = match crate::models::dice_slot::DiceSlot::new(i) {
                                Ok(s) => s,
                                Err(_) => return false,
                            };
                            mapping_clone.get(slot).is_none()
                        });
                        match empty {
                            Some(idx) => idx,
                            None => break,
                        }
                    };

                    let device_name = device.name.clone();
                    let _ = sender.send(DiceServiceEvent::Connecting {
                        name: device_name.clone(),
                        color: device_color,
                    });
                    let connect_manager = manager.clone();
                    match connect_manager.connect(&device).await {
                        Ok(dice) => {
                            let slot = match crate::models::dice_slot::DiceSlot::new(slot_idx) {
                                Ok(s) => s,
                                Err(error) => {
                                    debug!(error = %error, "invalid slot index");
                                    continue;
                                }
                            };
                            if let Err(error) = mapping.assign(slot, dice) {
                                debug!(error = %error, "failed to assign dice to slot");
                                continue;
                            }
                            if let Ok(mut names) = assigned_names.lock() {
                                names.push(device_name.clone());
                            }
                            info!(slot = slot_idx, device = %device_name, "dice assigned");
                            let _ = sender.send(DiceServiceEvent::DiceAssigned {
                                slot: slot_idx,
                                name: device_name.clone(),
                                color: device.color().unwrap_or(dice_rs::DiceColor::Black),
                            });
                            if mapping.is_complete() {
                                break;
                            }
                        }
                        Err(error) => {
                            debug!(error = %error, device = %device_name, "connection failed");
                            let _ = sender.send(DiceServiceEvent::DiceConnectionFailed {
                                name: device_name,
                                error: error.to_string(),
                            });
                        }
                    }
                }

                if mapping.is_complete() {
                    let _ = sender.send(DiceServiceEvent::AllSlotsAssigned);
                    break;
                }

                // If all discovered devices are either assigned or blacklisted,
                // clear the blacklist so swapped-out dice can be retried.
                let all_excluded = {
                    let names = assigned_names.lock().map(|n| n.clone()).unwrap_or_default();
                    let blacklist = blacklisted_names.lock().map(|b| b.clone()).unwrap_or_default();
                    device_names.iter().all(|n| names.contains(n) || blacklist.contains(n))
                };
                if all_excluded && !device_names.is_empty() {
                    debug!("all devices excluded, clearing blacklist for retry");
                    if let Ok(mut blacklist) = blacklisted_names.lock() {
                        blacklist.clear();
                    }
                }

                // Wait before retrying
                tokio::time::sleep(retry_interval).await;
            }
            is_scanning.store(false, Ordering::SeqCst);
        });
    }

    /// Disconnect all dice and clear the slot mapping.
    pub async fn disconnect_all(&self) -> Result<()> {
        let assigned = self.mapping.assigned();
        for (slot, dice) in assigned {
            if let Err(error) = dice.disconnect().await {
                debug!(slot = %slot.get(), error = %error, "disconnect failed");
            }
        }
        self.mapping.clear();
        if let Ok(mut names) = self.assigned_names.lock() {
            names.clear();
        }
        if let Ok(mut blacklist) = self.blacklisted_names.lock() {
            blacklist.clear();
        }
        Ok(())
    }

    /// Swap the dice in a specific slot for a different one.
    ///
    /// Disconnects the current dice, removes it from the mapping, and
    /// restarts scanning to connect a replacement. The freed slot will
    /// be filled by the next available dice.
    pub async fn swap_dice(&self, slot: crate::models::dice_slot::DiceSlot) -> Result<()> {
        if let Some(dice) = self.mapping.get(slot) {
            let name = dice.name().to_string();
            debug!(slot = slot.get(), device = %name, "swapping dice");
            if let Err(error) = dice.disconnect().await {
                debug!(slot = slot.get(), error = %error, "disconnect failed during swap");
            }
            self.mapping.remove(&name);
            if let Ok(mut names) = self.assigned_names.lock() {
                names.retain(|n| n != &name);
            }
            if let Ok(mut blacklist) = self.blacklisted_names.lock() {
                if !blacklist.contains(&name) {
                    blacklist.push(name);
                }
            }
        }
        // Restart scanning to find a replacement
        self.scan_and_connect();
        Ok(())
    }

    /// Handle a dice disconnection: remove from mapping.
    /// Returns the slot that was freed, if any.
    pub fn handle_disconnect(&self, dice_name: &str) -> Option<crate::models::dice_slot::DiceSlot> {
        let slot = self.mapping.remove(dice_name);
        if let Ok(mut names) = self.assigned_names.lock() {
            names.retain(|n| n != dice_name);
        }
        slot
    }

    /// Get the dice handle for a specific slot.
    pub fn get_dice(&self, slot: crate::models::dice_slot::DiceSlot) -> Option<Dice> {
        self.mapping.get(slot)
    }

    /// Check if all 5 slots are assigned.
    pub fn is_complete(&self) -> bool {
        self.mapping.is_complete()
    }

    /// Get the number of assigned slots.
    pub fn assigned_count(&self) -> usize {
        self.mapping.assigned_count()
    }

    /// Attempt to reconnect a disconnected dice.
    pub async fn reconnect(&self, dice: &Dice) -> Result<()> {
        self.manager.reconnect(dice).await.map_err(|e| YahtzeeError::Ble(e.to_string()))?;
        Ok(())
    }

    /// Attempt to reconnect all pending dice.
    ///
    /// Takes a list of (slot, device_name) pairs for pending reconnections.
    /// For each, retrieves the Dice handle from the mapping (kept alive
    /// after disconnect) and calls `manager.reconnect()`.
    ///
    /// Returns the list of slots that were successfully reconnected.
    pub async fn reconnect_pending(&self, pending: &[(crate::models::dice_slot::DiceSlot, String)]) -> Vec<crate::models::dice_slot::DiceSlot> {
        let mut reconnected = Vec::new();
        for (slot, _name) in pending {
            if let Some(dice) = self.mapping.get(*slot) {
                match self.manager.reconnect(&dice).await {
                    Ok(()) => {
                        info!(slot = slot.get(), "dice reconnected successfully");
                        reconnected.push(*slot);
                    }
                    Err(error) => {
                        debug!(slot = slot.get(), error = %error, "reconnect attempt failed");
                    }
                }
            }
        }
        reconnected
    }
}
