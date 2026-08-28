use dice_rs::DiceColor;
use glam::Quat;

/// Orientation state for smooth interpolation of dice rotation and color.
///
/// Tracks the current rendered orientation, the target orientation derived
/// from accelerometer data, the dice color, and a continuously advancing
/// spin angle used for idle rotation animation.
#[derive(Clone)]
pub struct OrientationState {
    /// The orientation currently rendered on screen.
    /// Updated each frame by combining the target with the spin rotation.
    pub orientation: Quat,
    /// The target orientation derived from accelerometer gravity data.
    /// The dice smoothly interpolates toward this orientation.
    pub target: Quat,
    /// Physical color of the dice surface, used for 3D rendering.
    pub color: DiceColor,
    /// Continuously advancing angle (in radians) for idle Y-axis spin animation.
    /// Incremented each render frame to produce smooth rotation.
    pub spin_angle: f32,
    /// Whether the idle Y-axis spin animation is enabled.
    pub rotation_enabled: bool,
}

impl Default for OrientationState {
    fn default() -> Self {
        Self {
            orientation: Quat::IDENTITY,
            target: Quat::IDENTITY,
            color: DiceColor::Black,
            spin_angle: 0.0,
            rotation_enabled: true,
        }
    }
}
