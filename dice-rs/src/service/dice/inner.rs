use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock as StdRwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU8;

use btleplug::api::Characteristic;
use tokio::sync::Notify;
use tokio::sync::broadcast;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;

use crate::ble::transport::BtleplugPeripheralWrapper;
use crate::model::acceleration::AccelerationOffset;
use crate::model::dice::DiceColor;
use crate::service::dice::event::DiceEvent;
use crate::service::led_throttle_state::LedThrottleState;

/// Internal shared state for a connected dice.
/// Stored behind `Arc` so all `Dice` clones share the same state.
pub struct DiceInner {
    /// Advertised device name (e.g. "GoDice_7D8E7D_O_v04").
    pub name: String,
    /// BLE peripheral for write/subscribe operations.
    pub peripheral: BtleplugPeripheralWrapper,
    /// Write characteristic (NUS RX).
    pub write_char: Characteristic,
    /// Notify characteristic (NUS TX).
    pub notify_char: Characteristic,
    /// Broadcast sender for `DiceEvent` stream.
    pub event_sender: broadcast::Sender<DiceEvent>,
    /// Current dice type stored as `AtomicU8` for lock-free reads.
    /// Converted to `DiceType` via `TryFrom<u8>` at use site.
    /// This avoids async lock overhead in the notification task hot path.
    /// `Arc`-wrapped so it can be cloned into the notification task.
    pub dice_type: Arc<AtomicU8>,
    /// FIFO queue of pending battery level request senders.
    pub pending_battery: Arc<Mutex<VecDeque<Sender<u8>>>>,
    /// FIFO queue of pending dice color request senders.
    pub pending_color: Arc<Mutex<VecDeque<Sender<DiceColor>>>>,
    /// FIFO queue of pending calibration request senders.
    pub pending_calibration: Arc<Mutex<VecDeque<Sender<bool>>>>,
    /// JoinHandle of the notification parsing task.
    /// Aborted on disconnect/reconnect to prevent orphaned tasks.
    pub notification_handle: Mutex<Option<JoinHandle<()>>>,
    /// JoinHandle of the connection monitor task.
    /// Aborted on disconnect/reconnect to prevent orphaned tasks.
    pub monitor_handle: Mutex<Option<JoinHandle<()>>>,
    /// LED write throttle state for coalescing rapid `set_leds` calls.
    pub led_throttle: Mutex<LedThrottleState>,
    /// JoinHandle of the LED debounce task.
    pub led_debounce_handle: Mutex<Option<JoinHandle<()>>>,
    /// Notify the LED debounce task that a new color is pending.
    pub led_notify: Arc<Notify>,
    /// Software calibration offset applied to accelerometer readings
    /// before face value interpretation. `None` when no software
    /// calibration has been performed.
    ///
    /// Uses `std::sync::RwLock` (not `tokio::sync::RwLock`) because the
    /// lock is only held for a trivial copy — never across `.await`.
    /// This avoids unnecessary task-scheduling overhead on every
    /// sensor event in the notification task.
    /// `Arc`-wrapped so it can be cloned into the notification task.
    pub calibration_offset: Arc<StdRwLock<Option<AccelerationOffset>>>,
    /// Last known charging state, updated by the notification task.
    /// `Arc`-wrapped so it can be cloned into the notification task.
    pub charging_state: Arc<AtomicBool>,
}
