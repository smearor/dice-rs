use crate::model::acceleration::Acceleration;
use crate::model::acceleration_offset::AccelerationOffset;
use crate::model::dice_type::DiceType;
use crate::model::face::FaceValue;

/// Finds the closest reference vector for the given acceleration and dice type.
/// Returns the reference vector as `[i32; 3]`.
pub fn closest_vector(acceleration: Acceleration, dice_type: DiceType) -> [i32; 3] {
    let table = dice_type.vector_table();
    let mut min_distance = i32::MAX;
    let mut best = (0i32, 0i32, 0i32);
    for &(rx, ry, rz) in table {
        let distance = acceleration.squared_distance_to(rx, ry, rz);
        if distance < min_distance {
            min_distance = distance;
            best = (rx, ry, rz);
        }
    }
    [best.0, best.1, best.2]
}

/// Determines the face value from accelerometer data for a given dice type.
///
/// If an `AccelerationOffset` is provided, it is subtracted from the
/// raw acceleration before distance calculation.
pub fn interpret(acceleration: Acceleration, dice_type: DiceType, offset: Option<AccelerationOffset>) -> FaceValue {
    let corrected = offset.map_or(acceleration, |o| o.apply(acceleration));
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
    FaceValue::new(mapped).unwrap_or(FaceValue::new(1).expect("fallback face value"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpret_d6_face1() {
        let accel = Acceleration { x: -64, y: 0, z: 0 };
        let face = interpret(accel, DiceType::D6, None);
        assert_eq!(face.get(), 1);
    }

    #[test]
    fn interpret_d6_face6() {
        let accel = Acceleration { x: 64, y: 0, z: 0 };
        let face = interpret(accel, DiceType::D6, None);
        assert_eq!(face.get(), 6);
    }

    #[test]
    fn interpret_d6_face2() {
        let accel = Acceleration { x: 0, y: 0, z: 64 };
        let face = interpret(accel, DiceType::D6, None);
        assert_eq!(face.get(), 2);
    }

    #[test]
    fn interpret_d20_face1() {
        let accel = Acceleration { x: -64, y: 0, z: -22 };
        let face = interpret(accel, DiceType::D20, None);
        assert_eq!(face.get(), 1);
    }

    #[test]
    fn interpret_d20_face20() {
        let accel = Acceleration { x: 64, y: 0, z: 22 };
        let face = interpret(accel, DiceType::D20, None);
        assert_eq!(face.get(), 20);
    }

    #[test]
    fn interpret_d6_with_offset() {
        let offset = Some(AccelerationOffset { dx: 2, dy: -1, dz: -1 });
        let accel = Acceleration { x: 2, y: -1, z: 63 };
        let face = interpret(accel, DiceType::D6, offset);
        assert_eq!(face.get(), 2);
    }

    #[test]
    fn interpret_d6_nearby_vector() {
        let accel = Acceleration { x: -60, y: 4, z: -3 };
        let face = interpret(accel, DiceType::D6, None);
        assert_eq!(face.get(), 1);
    }

    #[test]
    fn closest_vector_d6() {
        let accel = Acceleration { x: 0, y: 0, z: 64 };
        let closest = closest_vector(accel, DiceType::D6);
        assert_eq!(closest, [0, 0, 64]);
    }
}
