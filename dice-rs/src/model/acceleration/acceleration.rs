use crate::ble::parse_error::ParseError;
use crate::error::DiceError;
use crate::model::acceleration::offset::AccelerationOffset;
use crate::model::dice::DiceType;
use crate::model::face::FaceValue;
use crate::model::vec_u8::VecU8;
use serde::Deserialize;
use serde::Serialize;

/// Raw 3-axis accelerometer data from the dice.
///
/// Extracted as three signed 8-bit integers (`i8`) from the notification
/// payload. The Python API uses `struct.unpack(">bbb", xyz_bytes)` and
/// the JS API uses `data.getInt8(startByte)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Acceleration {
    /// X-axis acceleration.
    pub x: i8,
    /// Y-axis acceleration.
    pub y: i8,
    /// Z-axis acceleration.
    pub z: i8,
}

impl Acceleration {
    /// Compute the squared Euclidean distance to a reference vector.
    ///
    /// Uses `i32` internally to avoid overflow when squaring `i8` values.
    pub fn squared_distance_to(&self, rx: i32, ry: i32, rz: i32) -> i32 {
        let dx = self.x as i32 - rx;
        let dy = self.y as i32 - ry;
        let dz = self.z as i32 - rz;
        dx * dx + dy * dy + dz * dz
    }

    /// Find the closest reference vector from the dice type's vector table.
    ///
    /// Returns the reference vector as `[i32; 3]` that has the smallest
    /// squared Euclidean distance to this acceleration.
    pub fn closest_reference_vector(&self, dice_type: DiceType) -> [i32; 3] {
        let table = dice_type.vector_table();
        let mut min_distance = i32::MAX;
        let mut best = (0i32, 0i32, 0i32);
        for &(rx, ry, rz) in table {
            let distance = self.squared_distance_to(rx, ry, rz);
            if distance < min_distance {
                min_distance = distance;
                best = (rx, ry, rz);
            }
        }
        [best.0, best.1, best.2]
    }

    /// Compute the offset between this measured acceleration and the
    /// expected ideal gravity vector for the given dice type.
    ///
    /// The expected vector is the closest reference vector from the
    /// dice type's vector table. The offset is `measured - expected`.
    pub fn offset_to(&self, dice_type: DiceType) -> AccelerationOffset {
        let expected = self.closest_reference_vector(dice_type);
        AccelerationOffset {
            dx: self.x.saturating_sub(expected[0] as i8),
            dy: self.y.saturating_sub(expected[1] as i8),
            dz: self.z.saturating_sub(expected[2] as i8),
        }
    }

    /// Apply a calibration offset to this acceleration, clamping to i8 range.
    pub fn corrected_by(&self, offset: AccelerationOffset) -> Self {
        Self {
            x: self.x.saturating_sub(offset.dx),
            y: self.y.saturating_sub(offset.dy),
            z: self.z.saturating_sub(offset.dz),
        }
    }

    /// Interpret this acceleration as a face value for the given dice type.
    ///
    /// If an `AccelerationOffset` is provided, it is subtracted from the
    /// raw acceleration before distance calculation. The face value is
    /// determined by finding the closest reference vector in the dice
    /// type's vector table and applying its transform mapping.
    pub fn interpret_to_face(&self, dice_type: DiceType, offset: Option<AccelerationOffset>) -> Result<FaceValue, DiceError> {
        let corrected = offset.map_or(*self, |o| self.corrected_by(o));
        let table = dice_type.vector_table();
        let transform = dice_type.transform();

        let mut min_distance = i32::MAX;
        let mut best_index = 0;
        for (index, &(rx, ry, rz)) in table.iter().enumerate() {
            let distance = corrected.squared_distance_to(rx, ry, rz);
            if distance < min_distance {
                min_distance = distance;
                best_index = index;
            }
        }

        let raw_value = (best_index + 1) as u8;
        let mapped = transform.map_or(raw_value, |t| t[best_index]);
        FaceValue::new(mapped)
    }
}

/// Parse a `VecU8` byte buffer into `Acceleration`, returning an error if
/// the buffer contains fewer than 3 bytes.
impl TryFrom<&VecU8> for Acceleration {
    type Error = ParseError;

    fn try_from(buf: &VecU8) -> Result<Self, Self::Error> {
        Self::try_from(buf.as_slice())
    }
}

/// Parse a byte slice into `Acceleration`, returning an error if
/// the slice contains fewer than 3 bytes.
impl TryFrom<&[u8]> for Acceleration {
    type Error = ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 3 {
            return Err(ParseError::TruncatedPacket {
                expected: 3,
                actual: bytes.len(),
            });
        }
        Ok(Self {
            x: bytes[0] as i8,
            y: bytes[1] as i8,
            z: bytes[2] as i8,
        })
    }
}

impl std::fmt::Display for Acceleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_slice_valid() {
        let bytes: &[u8] = &[0x80, 0x40, 0x00];
        let accel = Acceleration::try_from(bytes).unwrap();
        assert_eq!(accel.x, -128);
        assert_eq!(accel.y, 64);
        assert_eq!(accel.z, 0);
    }

    #[test]
    fn try_from_vec_u8_valid() {
        let buf = VecU8::from_slice(&[0x80, 0x40, 0x00]);
        let accel = Acceleration::try_from(&buf).unwrap();
        assert_eq!(accel.x, -128);
        assert_eq!(accel.y, 64);
        assert_eq!(accel.z, 0);
    }

    #[test]
    fn try_from_slice_too_short() {
        let bytes: &[u8] = &[0x01, 0x02];
        let result = Acceleration::try_from(bytes);
        assert!(result.is_err());
    }

    #[test]
    fn try_from_vec_u8_too_short() {
        let buf = VecU8::from_slice(&[0x01]);
        let result = Acceleration::try_from(&buf);
        assert!(result.is_err());
    }

    #[test]
    fn squared_distance_zero() {
        let accel = Acceleration { x: 10, y: 20, z: 30 };
        assert_eq!(accel.squared_distance_to(10, 20, 30), 0);
    }

    #[test]
    fn squared_distance_nonzero() {
        let accel = Acceleration { x: 0, y: 0, z: 0 };
        assert_eq!(accel.squared_distance_to(1, 2, 3), 14);
    }

    #[test]
    fn display() {
        let accel = Acceleration { x: -1, y: 2, z: 3 };
        assert_eq!(accel.to_string(), "(-1, 2, 3)");
    }

    #[test]
    fn offset_to_d6() {
        let accel = Acceleration { x: 2, y: -1, z: 63 };
        let offset = accel.offset_to(DiceType::D6);
        assert_eq!(offset, AccelerationOffset { dx: 2, dy: -1, dz: -1 });
    }

    #[test]
    fn corrected_by_removes_drift() {
        let offset = AccelerationOffset { dx: 2, dy: -1, dz: -1 };
        let accel = Acceleration { x: 2, y: -1, z: 63 };
        let corrected = accel.corrected_by(offset);
        assert_eq!(corrected, Acceleration { x: 0, y: 0, z: 64 });
    }

    #[test]
    fn corrected_by_saturates() {
        let offset = AccelerationOffset { dx: 100, dy: -100, dz: 0 };
        let accel = Acceleration { x: 1, y: -1, z: 50 };
        let corrected = accel.corrected_by(offset);
        assert_eq!(corrected, Acceleration { x: -99, y: 99, z: 50 });
    }

    #[test]
    fn closest_reference_vector_d6() {
        let accel = Acceleration { x: 0, y: 0, z: 64 };
        let closest = accel.closest_reference_vector(DiceType::D6);
        assert_eq!(closest, [0, 0, 64]);
    }

    #[test]
    fn interpret_to_face_d6_face1() {
        let accel = Acceleration { x: -64, y: 0, z: 0 };
        let face = accel.interpret_to_face(DiceType::D6, None).unwrap();
        assert_eq!(face.get(), 1);
    }

    #[test]
    fn interpret_to_face_d6_face6() {
        let accel = Acceleration { x: 64, y: 0, z: 0 };
        let face = accel.interpret_to_face(DiceType::D6, None).unwrap();
        assert_eq!(face.get(), 6);
    }

    #[test]
    fn interpret_to_face_d6_face2() {
        let accel = Acceleration { x: 0, y: 0, z: 64 };
        let face = accel.interpret_to_face(DiceType::D6, None).unwrap();
        assert_eq!(face.get(), 2);
    }

    #[test]
    fn interpret_to_face_d20_face1() {
        let accel = Acceleration { x: -64, y: 0, z: -22 };
        let face = accel.interpret_to_face(DiceType::D20, None).unwrap();
        assert_eq!(face.get(), 1);
    }

    #[test]
    fn interpret_to_face_d20_face20() {
        let accel = Acceleration { x: 64, y: 0, z: 22 };
        let face = accel.interpret_to_face(DiceType::D20, None).unwrap();
        assert_eq!(face.get(), 20);
    }

    #[test]
    fn interpret_to_face_d6_with_offset() {
        let offset = Some(AccelerationOffset { dx: 2, dy: -1, dz: -1 });
        let accel = Acceleration { x: 2, y: -1, z: 63 };
        let face = accel.interpret_to_face(DiceType::D6, offset).unwrap();
        assert_eq!(face.get(), 2);
    }

    #[test]
    fn interpret_to_face_d6_nearby_vector() {
        let accel = Acceleration { x: -60, y: 4, z: -3 };
        let face = accel.interpret_to_face(DiceType::D6, None).unwrap();
        assert_eq!(face.get(), 1);
    }
}
