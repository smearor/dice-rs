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
    /// RGB color of the dice surface, set from `DiceColor`.
    /// Stored as `[r, g, b]` with values in the range `0.0..=1.0`.
    pub color: [f32; 3],
    /// Continuously advancing angle (in radians) for idle Y-axis spin animation.
    /// Incremented each render frame to produce smooth rotation.
    pub spin_angle: f32,
}

impl Default for OrientationState {
    fn default() -> Self {
        Self {
            orientation: Quat::IDENTITY,
            target: Quat::IDENTITY,
            color: [0.95, 0.95, 0.95],
            spin_angle: 0.0,
        }
    }
}
