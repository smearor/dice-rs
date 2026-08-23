/// D10 shell transform: maps D20 vector index → D10 face value.
/// Ported from JS API d10Transform.
pub const D10_TRANSFORM: [u8; 20] = [8, 2, 6, 1, 4, 3, 9, 0, 7, 5, 5, 7, 0, 9, 3, 4, 1, 6, 2, 8];

/// D10X shell transform: maps D20 vector index → D10X face value.
/// Ported from JS API d10XTransform.
pub const D10X_TRANSFORM: [u8; 20] = [80, 20, 60, 10, 40, 30, 90, 0, 70, 50, 50, 70, 0, 90, 30, 40, 10, 60, 20, 80];

/// D4 shell transform: maps D24 vector index → D4 face value.
/// Ported from JS API d4Transform.
pub const D4_TRANSFORM: [u8; 24] = [3, 1, 4, 1, 4, 4, 1, 4, 2, 3, 1, 1, 1, 4, 2, 3, 3, 2, 2, 2, 4, 1, 3, 2];

/// D8 shell transform: maps D24 vector index → D8 face value.
/// Ported from JS API d8Transform.
pub const D8_TRANSFORM: [u8; 24] = [3, 3, 6, 1, 2, 8, 1, 1, 4, 7, 5, 5, 4, 4, 2, 5, 7, 7, 8, 2, 8, 3, 6, 6];

/// D12 shell transform: maps D24 vector index → D12 face value.
/// Ported from JS API d12Transform.
pub const D12_TRANSFORM: [u8; 24] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
