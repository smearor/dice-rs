use uuid::Uuid;

/// Nordic UART Service (NUS) UUID.
pub const NUS_SERVICE_UUID: Uuid = Uuid::from_u128(0x6e400001_b5a3_f393_e0a9_e50e24dcca9e);

/// NUS Write characteristic UUID (Host → Dice).
pub const NUS_WRITE_CHAR_UUID: Uuid = Uuid::from_u128(0x6e400002_b5a3_f393_e0a9_e50e24dcca9e);

/// NUS Notify characteristic UUID (Dice → Host).
pub const NUS_NOTIFY_CHAR_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_uuid_matches_gatt_spec() {
        assert_eq!(NUS_SERVICE_UUID.to_string(), "6e400001-b5a3-f393-e0a9-e50e24dcca9e");
    }

    #[test]
    fn write_char_uuid_matches_gatt_spec() {
        assert_eq!(NUS_WRITE_CHAR_UUID.to_string(), "6e400002-b5a3-f393-e0a9-e50e24dcca9e");
    }

    #[test]
    fn notify_char_uuid_matches_gatt_spec() {
        assert_eq!(NUS_NOTIFY_CHAR_UUID.to_string(), "6e400003-b5a3-f393-e0a9-e50e24dcca9e");
    }
}
