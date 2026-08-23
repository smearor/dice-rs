use crate::model::acceleration::Acceleration;
use crate::model::dice_type::DiceType;
use crate::service::interpreter::interpret::closest_vector;

/// Software calibration offset computed from a resting accelerometer sample.
///
/// When the firmware does not support hardware calibration via BLE,
/// `calibrate_software()` captures the current XYZ reading and computes
/// the deviation from the expected gravity vector. The offset is subtracted
/// from all subsequent accelerometer readings before face value interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AccelerationOffset {
    /// X-axis deviation from the expected resting vector.
    pub dx: i8,
    /// Y-axis deviation from the expected resting vector.
    pub dy: i8,
    /// Z-axis deviation from the expected resting vector.
    pub dz: i8,
}

impl AccelerationOffset {
    /// Compute the offset between a measured acceleration and the
    /// expected ideal gravity vector for the given dice type.
    ///
    /// The expected vector is the closest reference vector from the
    /// dice type's vector table. The offset is `measured - expected`.
    pub fn from_measured(acceleration: Acceleration, dice_type: DiceType) -> Self {
        let expected = closest_vector(acceleration, dice_type);
        Self {
            dx: acceleration.x.saturating_sub(expected[0] as i8),
            dy: acceleration.y.saturating_sub(expected[1] as i8),
            dz: acceleration.z.saturating_sub(expected[2] as i8),
        }
    }

    /// Apply the offset to an acceleration reading, clamping to i8 range.
    pub fn apply(&self, acceleration: Acceleration) -> Acceleration {
        Acceleration {
            x: acceleration.x.saturating_sub(self.dx),
            y: acceleration.y.saturating_sub(self.dy),
            z: acceleration.z.saturating_sub(self.dz),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_measured_d6() {
        let acceleration = Acceleration { x: 2, y: -1, z: 63 };
        let offset = AccelerationOffset::from_measured(acceleration, DiceType::D6);
        assert_eq!(offset, AccelerationOffset { dx: 2, dy: -1, dz: -1 });
    }

    #[test]
    fn apply_corrects_drift() {
        let offset = AccelerationOffset { dx: 2, dy: -1, dz: -1 };
        let acceleration = Acceleration { x: 2, y: -1, z: 63 };
        let corrected = offset.apply(acceleration);
        assert_eq!(corrected, Acceleration { x: 0, y: 0, z: 64 });
    }

    #[test]
    fn apply_saturates() {
        let offset = AccelerationOffset { dx: 100, dy: -100, dz: 0 };
        let acceleration = Acceleration { x: 1, y: -1, z: 50 };
        let corrected = offset.apply(acceleration);
        assert_eq!(corrected, Acceleration { x: -99, y: 99, z: 50 });
    }
}
