use smallvec::SmallVec;

/// Inline capacity for the most common BLE payload size (3-byte acceleration data).
const INLINE_CAPACITY: usize = 3;

/// A byte buffer newtype backed by `SmallVec` that provides panic-free access.
///
/// Small enough payloads (up to `INLINE_CAPACITY` bytes) are stored inline on
/// the stack without heap allocation. All indexing methods return `Option`,
/// eliminating the risk of runtime panics from out-of-bounds access.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecU8(SmallVec<[u8; INLINE_CAPACITY]>);

impl VecU8 {
    /// Create an empty buffer.
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    /// Create a buffer from a byte slice.
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(SmallVec::from_slice(slice))
    }

    /// Returns the number of bytes in the buffer.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the byte at the given index, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    /// Returns the byte at the given index as `i8`, or `None` if out of bounds.
    pub fn get_i8(&self, index: usize) -> Option<i8> {
        self.0.get(index).map(|b| *b as i8)
    }

    /// Returns a slice of the underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Default for VecU8 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&[u8]> for VecU8 {
    fn from(slice: &[u8]) -> Self {
        Self::from_slice(slice)
    }
}

impl std::fmt::Display for VecU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, byte) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "0x{byte:02X}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let buf = VecU8::new();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn from_slice_preserves_data() {
        let buf = VecU8::from_slice(&[0x01, 0x02, 0x03]);
        assert_eq!(buf.len(), 3);
        assert_eq!(buf.get(0), Some(0x01));
        assert_eq!(buf.get(1), Some(0x02));
        assert_eq!(buf.get(2), Some(0x03));
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let buf = VecU8::from_slice(&[0x01]);
        assert_eq!(buf.get(1), None);
        assert_eq!(buf.get(usize::MAX), None);
    }

    #[test]
    fn get_i8_signed_conversion() {
        let buf = VecU8::from_slice(&[0x80, 0x7F, 0x00]);
        assert_eq!(buf.get_i8(0), Some(-128));
        assert_eq!(buf.get_i8(1), Some(127));
        assert_eq!(buf.get_i8(2), Some(0));
    }

    #[test]
    fn get_i8_out_of_bounds_returns_none() {
        let buf = VecU8::from_slice(&[0x01]);
        assert_eq!(buf.get_i8(5), None);
    }

    #[test]
    fn display() {
        let buf = VecU8::from_slice(&[0x01, 0xFF, 0x42]);
        assert_eq!(buf.to_string(), "[0x01, 0xFF, 0x42]");
    }

    #[test]
    fn display_empty() {
        let buf = VecU8::new();
        assert_eq!(buf.to_string(), "[]");
    }

    #[test]
    fn from_slice_inline_no_heap_alloc() {
        // 3 bytes fits inline capacity — no heap allocation
        let buf = VecU8::from_slice(&[10, 20, 30]);
        assert_eq!(buf.as_slice(), &[10, 20, 30]);
    }

    #[test]
    fn from_slice_spills_to_heap() {
        // 10 bytes exceeds inline capacity — spills to heap transparently
        let buf = VecU8::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(buf.len(), 10);
        assert_eq!(buf.get(9), Some(9));
    }
}
