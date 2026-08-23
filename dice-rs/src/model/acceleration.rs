/// Raw 3-axis accelerometer data from the dice.
///
/// Extracted as three signed 8-bit integers (`i8`) from the notification
/// payload. The Python API uses `struct.unpack(">bbb", xyz_bytes)` and
/// the JS API uses `data.getInt8(startByte)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Acceleration {
    /// X-axis acceleration.
    pub x: i8,
    /// Y-axis acceleration.
    pub y: i8,
    /// Z-axis acceleration.
    pub z: i8,
}

impl Acceleration {
    /// Parse three bytes as signed XYZ accelerometer data.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            x: bytes[0] as i8,
            y: bytes[1] as i8,
            z: bytes[2] as i8,
        }
    }

    /// Compute the squared Euclidean distance to a reference vector.
    ///
    /// Uses `i32` internally to avoid overflow when squaring `i8` values.
    pub fn squared_distance_to(&self, rx: i32, ry: i32, rz: i32) -> i32 {
        let dx = self.x as i32 - rx;
        let dy = self.y as i32 - ry;
        let dz = self.z as i32 - rz;
        dx * dx + dy * dy + dz * dz
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
    fn from_bytes() {
        let accel = Acceleration::from_bytes(&[0x80, 0x40, 0x00]);
        assert_eq!(accel.x, -128);
        assert_eq!(accel.y, 64);
        assert_eq!(accel.z, 0);
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
}
