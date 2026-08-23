use crate::model::led::LedColor;

/// Coalescing debounce state for LED write commands.
///
/// When `set_leds` is called repeatedly within `LED_DEBOUNCE_MS`,
/// only the most recent color is written to the BLE transport.
/// A pending write is deferred until no new `set_leds` call arrives
/// for the debounce window, then flushed by a background task.
pub struct LedThrottleState {
    /// Most recent LED colors requested.
    pub pending: Option<(LedColor, LedColor)>,
    /// Instant of the last `set_leds` call.
    pub last_update: Option<tokio::time::Instant>,
}
