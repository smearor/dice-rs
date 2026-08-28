use dice_rs::model::acceleration::Acceleration;
use dice_rs::model::battery_level::BatteryLevel;
use dice_rs::model::charging_state::ChargingState;
use dice_rs::model::face::FaceValue;

/// UI update commands sent from the tokio event loop to the GTK main thread.
///
/// `EventController` produces these variants on a background tokio task and
/// forwards them via a `std::sync::mpsc::Sender`. The GTK side (in `DiceRow`)
/// polls the corresponding receiver with `glib::timeout_add_local` and applies
/// each update to the appropriate widgets. This keeps all widget mutation on
/// the GTK main thread while dice I/O runs off-thread.
pub enum UiUpdate {
    /// The dice started rolling — reset face display to the rolling state.
    Rolling,
    /// The dice settled on a face with full stability.
    Stable {
        /// The resolved face value after the roll.
        face: FaceValue,
        /// The acceleration vector at rest, used to orient the 3D model.
        acceleration: Acceleration,
    },
    /// The dice settled but is tilted (not flat on a face).
    TiltStable {
        /// The resolved face value despite the tilt.
        face: FaceValue,
        /// The acceleration vector at rest, used to orient the 3D model.
        acceleration: Acceleration,
    },
    /// The dice reported a stable face but the reading is likely spurious.
    FakeStable {
        /// The suspected face value.
        face: FaceValue,
        /// The acceleration vector at rest, used to orient the 3D model.
        acceleration: Acceleration,
    },
    /// The dice settled after being moved (not thrown).
    MoveStable {
        /// The resolved face value after the move.
        face: FaceValue,
        /// The acceleration vector at rest, used to orient the 3D model.
        acceleration: Acceleration,
    },
    /// The dice charging state changed.
    Charging {
        /// The new charging state (charging or not charging).
        state: ChargingState,
    },
    /// A single tap was detected on the dice.
    Tap,
    /// A double tap was detected on the dice.
    DoubleTap,
    /// The BLE connection to the dice was lost.
    Disconnected,
    /// A periodic battery level reading completed.
    BatteryLevel(
        /// The most recently reported battery level.
        BatteryLevel,
    ),
}
