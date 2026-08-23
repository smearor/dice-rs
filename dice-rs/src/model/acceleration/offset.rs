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
