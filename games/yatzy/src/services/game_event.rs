use crate::models::dice_slot::DiceSlot;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::services::led_effect::LedEffect;
use dice_rs::FaceValue;

/// High-level game events derived from BLE dice events.
///
/// These events represent the game-relevant interpretation of raw
/// `DiceEvent`s. The UI layer consumes these instead of raw BLE events.
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    /// A roll has started on one or more dice.
    RollStarted,
    /// All dice are now stable. Contains the face values for each slot.
    RollComplete {
        /// Face values indexed by slot (0-4). `None` if a dice didn't report.
        faces: [Option<FaceValue>; 5],
    },
    /// The roll timed out before all dice became stable.
    RollTimedOut {
        /// Partial face values received before timeout.
        faces: [Option<FaceValue>; 5],
    },
    /// A single dice became stable with a face value.
    DiceStable {
        /// The slot that became stable.
        slot: DiceSlot,
        /// The face value reported.
        face: FaceValue,
    },
    /// A single dice was picked up (started moving) during Holding phase.
    ///
    /// The UI can use this to auto-hold all other dice that were not picked up.
    DicePickedUp {
        /// The slot that was picked up.
        slot: DiceSlot,
    },
    /// A dice disconnected from its slot.
    DiceDisconnected {
        /// The slot that disconnected.
        slot: DiceSlot,
        /// The device name (for reconnection).
        name: String,
    },
    /// A dice reconnected to its slot.
    DiceReconnected {
        /// The slot that reconnected.
        slot: DiceSlot,
    },
    /// An LED effect should be applied (e.g. celebration).
    ApplyLedEffect {
        /// The effect to apply.
        effect: LedEffect,
    },
    /// The active player changed. LEDs should be updated.
    ActivePlayerChanged {
        /// The new active player's index.
        player_index: PlayerIndex,
        /// The new active player's color.
        color: PlayerColor,
    },
}
