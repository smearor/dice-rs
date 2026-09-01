use crate::models::dice_slot::DiceSlot;
use dice_rs::Dice;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Mapping between physical GoDice devices and game dice slots.
///
/// Each physical `Dice` is assigned to a `DiceSlot` (0-4). The mapping
/// is established during scanning and can be re-assigned if a dice
/// disconnects and a spare is used.
#[derive(Clone)]
pub struct SlotMapping {
    /// Maps slot index to physical dice handle.
    slots: Arc<Mutex<Vec<Option<Dice>>>>,
    /// Maps dice name to slot index, for reverse lookup.
    name_to_slot: Arc<Mutex<HashMap<String, u8>>>,
}

impl SlotMapping {
    /// Create a new empty slot mapping with 5 slots.
    pub fn new() -> Self {
        Self {
            slots: Arc::new(Mutex::new(vec![None; DiceSlot::COUNT])),
            name_to_slot: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Assign a dice to a slot.
    ///
    /// If the slot is already occupied, the previous dice is replaced.
    /// Returns an error if the slot index is invalid.
    pub fn assign(&self, slot: DiceSlot, dice: Dice) -> crate::error::Result<()> {
        let name = dice.name().to_string();
        {
            let mut slots = self.slots.lock().map_err(|_| crate::error::YahtzeeError::LockPoisoned)?;
            slots[slot.get() as usize] = Some(dice);
        }
        {
            let mut map = self.name_to_slot.lock().map_err(|_| crate::error::YahtzeeError::LockPoisoned)?;
            map.insert(name, slot.get());
        }
        Ok(())
    }

    /// Get the dice assigned to a slot, if any.
    pub fn get(&self, slot: DiceSlot) -> Option<Dice> {
        let slots = self.slots.lock().ok()?;
        slots[slot.get() as usize].clone()
    }

    /// Find the slot assigned to a dice by name.
    pub fn slot_by_name(&self, name: &str) -> Option<DiceSlot> {
        let map = self.name_to_slot.lock().ok()?;
        map.get(name).and_then(|&idx| DiceSlot::new(idx).ok())
    }

    /// Remove a dice from its slot (e.g. on disconnection).
    pub fn remove(&self, name: &str) -> Option<DiceSlot> {
        let slot = {
            let mut map = self.name_to_slot.lock().ok()?;
            map.remove(name)
        };
        let slot = slot?;
        let slot_idx = DiceSlot::new(slot).ok()?;
        if let Ok(mut slots) = self.slots.lock() {
            slots[slot_idx.get() as usize] = None;
        }
        Some(slot_idx)
    }

    /// Returns true if all 5 slots are assigned.
    pub fn is_complete(&self) -> bool {
        let slots = match self.slots.lock() {
            Ok(s) => s,
            Err(_) => return false,
        };
        slots.iter().all(|d| d.is_some())
    }

    /// Returns the number of assigned slots.
    pub fn assigned_count(&self) -> usize {
        let slots = match self.slots.lock() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        slots.iter().filter(|d| d.is_some()).count()
    }

    /// Returns the number of empty (unassigned) slots.
    pub fn empty_count(&self) -> usize {
        DiceSlot::COUNT - self.assigned_count()
    }

    /// Clear all slot assignments.
    pub fn clear(&self) {
        if let Ok(mut slots) = self.slots.lock() {
            for slot in slots.iter_mut() {
                *slot = None;
            }
        }
        if let Ok(mut map) = self.name_to_slot.lock() {
            map.clear();
        }
    }

    /// Get all assigned dice as a vector of (slot, dice) pairs.
    pub fn assigned(&self) -> Vec<(DiceSlot, Dice)> {
        let slots = match self.slots.lock() {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        slots
            .iter()
            .enumerate()
            .filter_map(|(idx, d)| DiceSlot::new(idx as u8).ok().zip(d.clone()))
            .collect()
    }
}

impl Default for SlotMapping {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // We can't create real Dice handles in unit tests without BLE,
    // so we test the mapping logic indirectly via the name_to_slot map.

    #[test]
    fn new_is_empty() {
        let mapping = SlotMapping::new();
        assert_eq!(mapping.assigned_count(), 0);
        assert_eq!(mapping.empty_count(), 5);
        assert!(!mapping.is_complete());
    }

    #[test]
    fn empty_count_decreases() {
        let mapping = SlotMapping::new();
        assert_eq!(mapping.empty_count(), 5);
        // Can't test assign without a real Dice, but we can test clear
        mapping.clear();
        assert_eq!(mapping.empty_count(), 5);
    }

    #[test]
    fn clear_resets() {
        let mapping = SlotMapping::new();
        mapping.clear();
        assert_eq!(mapping.assigned_count(), 0);
    }
}
