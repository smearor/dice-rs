use crate::error::Result;
use crate::error::YahtzeeError;
use crate::services::slot_mapping::SlotMapping;
use dice_rs::Dice;
use dice_rs::DiceManager;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;

/// Events emitted by `DiceService` for the UI to react to.
#[derive(Debug, Clone)]
pub enum DiceServiceEvent {
    /// A scan was started.
    ScanStarted,
    /// The scan completed but no devices were found.
    NoDevicesFound,
    /// The scan found devices; `count` is the total number.
    DevicesFound(usize),
    /// A dice was successfully connected and assigned to a slot.
    DiceAssigned {
        /// The slot the dice was assigned to.
        slot: u8,
        /// The device name.
        name: String,
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

    /// Start a scan: discover devices, connect, and assign to slots.
    ///
    /// This method spawns a tokio task and returns immediately.
    /// Progress is reported via the event channel.
    pub fn scan_and_connect(&self) {
        let manager = self.manager.clone();
        let mapping = self.mapping.clone();
        let sender = self.event_sender.clone();
        let assigned_names = self.assigned_names.clone();

        tokio::spawn(async move {
            let _ = sender.send(DiceServiceEvent::ScanStarted);

            let scan_result = manager.scan().await;
            let devices = match scan_result {
                Ok(devices) => {
                    if devices.is_empty() {
                        let _ = sender.send(DiceServiceEvent::NoDevicesFound);
                        return;
                    }
                    let count = devices.len();
                    let _ = sender.send(DiceServiceEvent::DevicesFound(count));
                    devices
                }
                Err(error) => {
                    debug!(error = %error, "scan failed");
                    let _ = sender.send(DiceServiceEvent::ScanFailed(error.to_string()));
                    return;
                }
            };

            for device in devices {
                // Skip if already assigned
                let already_assigned = {
                    let names = match assigned_names.lock() {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    names.contains(&device.name)
                };
                if already_assigned {
                    continue;
                }

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
                        None => {
                            // All slots assigned
                            let _ = sender.send(DiceServiceEvent::AllSlotsAssigned);
                            return;
                        }
                    }
                };

                let device_name = device.name.clone();
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
                        let _ = sender.send(DiceServiceEvent::DiceAssigned {
                            slot: slot_idx,
                            name: device_name,
                        });
                        if mapping.is_complete() {
                            let _ = sender.send(DiceServiceEvent::AllSlotsAssigned);
                            return;
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
}
