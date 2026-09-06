use crate::models::dice_slot::DiceSlot;
use serde::Deserialize;
use serde::Serialize;

/// A request to reconnect a disconnected dice to its slot.
///
/// Created when a dice disconnects mid-game, containing the slot
/// it was assigned to and the device name needed for reconnection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconnectionRequest {
    /// The slot the dice was assigned to.
    slot: DiceSlot,
    /// The BLE device name of the disconnected dice.
    device_name: String,
}

impl ReconnectionRequest {
    /// Create a new reconnection request.
    pub fn new(slot: DiceSlot, device_name: impl Into<String>) -> Self {
        Self {
            slot,
            device_name: device_name.into(),
        }
    }

    /// Get the slot that needs reconnection.
    pub fn slot(&self) -> DiceSlot {
        self.slot
    }

    /// Get the device name for reconnection.
    pub fn device_name(&self) -> &str {
        &self.device_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_request() {
        let request = ReconnectionRequest::new(DiceSlot::new(2).unwrap(), "GoDice_abc123");
        assert_eq!(request.slot(), DiceSlot::new(2).unwrap());
        assert_eq!(request.device_name(), "GoDice_abc123");
    }

    #[test]
    fn slot_and_name_accessors() {
        let request = ReconnectionRequest::new(DiceSlot::new(0).unwrap(), "TestDevice");
        assert_eq!(request.slot().get(), 0);
        assert_eq!(request.device_name(), "TestDevice");
    }

    #[test]
    fn serialize_deserialize() {
        let request = ReconnectionRequest::new(DiceSlot::new(3).unwrap(), "GoDice_xyz");
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ReconnectionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, deserialized);
    }
}
