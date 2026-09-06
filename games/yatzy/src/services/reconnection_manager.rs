use crate::error::Result;
use crate::models::reconnection_request::ReconnectionRequest;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Manages reconnection of disconnected dice.
///
/// Tracks dice that have disconnected mid-game and provides methods
/// to attempt reconnection. The manager maintains a list of pending
/// reconnection requests keyed by slot.
pub struct ReconnectionManager {
    /// Pending reconnection requests, keyed by slot index.
    pending: Arc<Mutex<HashMap<u8, ReconnectionRequest>>>,
}

impl ReconnectionManager {
    /// Create a new reconnection manager with no pending requests.
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a dice disconnection for potential reconnection.
    ///
    /// Stores the slot and device name so that a reconnection attempt
    /// can be made later.
    pub fn register_disconnect(&self, request: ReconnectionRequest) {
        let slot_index = request.slot().get();
        if let Ok(mut pending) = self.pending.lock() {
            debug!(slot = slot_index, device = request.device_name(), "registered disconnect for reconnection");
            pending.insert(slot_index, request);
        }
    }

    /// Attempt to reconnect a dice for the given slot.
    ///
    /// Returns the device name if a pending request exists for the slot,
    /// or `None` if no reconnection is pending.
    pub fn pending_for_slot(&self, slot: crate::models::dice_slot::DiceSlot) -> Option<String> {
        let pending = self.pending.lock().ok()?;
        pending.get(&slot.get()).map(|r| r.device_name().to_string())
    }

    /// Mark a slot as reconnected, removing it from pending requests.
    #[allow(clippy::collapsible_if)]
    pub fn mark_reconnected(&self, slot: crate::models::dice_slot::DiceSlot) {
        if let Ok(mut pending) = self.pending.lock() {
            if pending.remove(&slot.get()).is_some() {
                info!(slot = slot.get(), "dice reconnected successfully");
            }
        }
    }

    /// Returns the number of pending reconnection requests.
    pub fn pending_count(&self) -> usize {
        self.pending.lock().map(|p| p.len()).unwrap_or(0)
    }

    /// Returns true if there are any pending reconnection requests.
    pub fn has_pending(&self) -> bool {
        self.pending_count() > 0
    }

    /// Returns a list of all pending reconnection requests.
    pub fn pending_requests(&self) -> Vec<ReconnectionRequest> {
        self.pending.lock().map(|p| p.values().cloned().collect()).unwrap_or_default()
    }

    /// Clear all pending reconnection requests.
    pub fn clear(&self) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.clear();
        }
    }

    /// Attempt to reconnect all pending dice using the provided reconnection function.
    ///
    /// The `reconnect_fn` is called for each pending request with the device name.
    /// Successfully reconnected dice are removed from the pending list.
    pub async fn attempt_reconnect_all<F, Fut>(&self, reconnect_fn: F) -> Result<()>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let requests: Vec<ReconnectionRequest> = self.pending_requests();
        for request in requests {
            let slot = request.slot();
            let device_name = request.device_name().to_string();
            match reconnect_fn(device_name.clone()).await {
                Ok(()) => {
                    self.mark_reconnected(slot);
                }
                Err(error) => {
                    warn!(slot = slot.get(), device = device_name, error = %error, "reconnection attempt failed");
                }
            }
        }
        Ok(())
    }
}

impl Default for ReconnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::YatzyError;
    use crate::models::dice_slot::DiceSlot;

    #[test]
    fn new_has_no_pending() {
        let manager = ReconnectionManager::new();
        assert!(!manager.has_pending());
        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn register_disconnect_adds_pending() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(2).unwrap(), "GoDice_abc"));
        assert!(manager.has_pending());
        assert_eq!(manager.pending_count(), 1);
    }

    #[test]
    fn pending_for_slot_returns_device_name() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(1).unwrap(), "GoDice_xyz"));
        let name = manager.pending_for_slot(DiceSlot::new(1).unwrap());
        assert_eq!(name.as_deref(), Some("GoDice_xyz"));
    }

    #[test]
    fn pending_for_slot_returns_none_if_not_pending() {
        let manager = ReconnectionManager::new();
        let name = manager.pending_for_slot(DiceSlot::new(0).unwrap());
        assert!(name.is_none());
    }

    #[test]
    fn mark_reconnected_removes_pending() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(3).unwrap(), "GoDice_test"));
        assert!(manager.has_pending());
        manager.mark_reconnected(DiceSlot::new(3).unwrap());
        assert!(!manager.has_pending());
    }

    #[test]
    fn clear_removes_all_pending() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(0).unwrap(), "A"));
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(1).unwrap(), "B"));
        assert_eq!(manager.pending_count(), 2);
        manager.clear();
        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn pending_requests_returns_all() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(0).unwrap(), "A"));
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(1).unwrap(), "B"));
        let requests = manager.pending_requests();
        assert_eq!(requests.len(), 2);
    }

    #[tokio::test]
    async fn attempt_reconnect_all_succeeds() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(0).unwrap(), "A"));
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(1).unwrap(), "B"));

        manager.attempt_reconnect_all(|_name| async move { Ok(()) }).await.unwrap();

        assert_eq!(manager.pending_count(), 0);
    }

    #[tokio::test]
    async fn attempt_reconnect_all_partial_failure() {
        let manager = ReconnectionManager::new();
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(0).unwrap(), "A"));
        manager.register_disconnect(ReconnectionRequest::new(DiceSlot::new(1).unwrap(), "B"));

        manager
            .attempt_reconnect_all(|name| async move {
                if name == "B" {
                    Err(YatzyError::ReconnectionFailed("B".to_string()))
                } else {
                    Ok(())
                }
            })
            .await
            .unwrap();

        assert_eq!(manager.pending_count(), 1);
        assert!(manager.pending_for_slot(DiceSlot::new(1).unwrap()).is_some());
    }
}
