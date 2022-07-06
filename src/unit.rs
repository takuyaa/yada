/// UnitID is an alias of `usize`.
pub type UnitID = usize;

/// The size of `Unit` (4).
pub const UNIT_SIZE: usize = std::mem::size_of::<u32>();

/// An unit represents an element in a double-array.
#[derive(Copy, Clone)]
pub struct Unit(u32);

/// Unit represents one node of a double array trie. The bit width of each node is 32-bits.
///
/// The bit layout of a non-leaf node:
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +---------------+-+-+-----------------------------------------+-+
/// |     LABEL     |H|E|                 OFFSET                  |I|
/// +---------------+-+-+-----------------------------------------+-+
///
///   LABEL                8-bits value that represents a label of the double array node.
///   HAS_LEAF (H)         1-bit flag that indicates whether the node has leaf nodes or not.
///   EXTEND_OFFSET (E)    1-bit flag that indicates whether the offset should be extended or not.
///   OFFSET               21-bits value that represents an offset of the double array node.
///   IS_LEAF (I)          1-bit flag that indicates whether the node is a leaf node or not.
///                        This flag is always 0 in this case.
///
/// The bit layout of a leaf node:
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-------------------------------------------------------------+-+
/// |                            VALUE                            |I|
/// +-------------------------------------------------------------+-+
///
///   VALUE                31-bits value that represents a value of the double array node.
///   IS_LEAF (I)          1-bit flag that indicates whether the node is a leaf node or not.
///                        This flag is always 1 in this case.
impl Unit {
    /// Creates a new Unit.
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new Unit from `value`.
    #[inline]
    pub fn from_u32(value: u32) -> Self {
        Self(value)
    }

    /// Returns an internal 32 bit integer.
    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Returns true if the unit have a leaf as a child unit. Otherwise, returns false.
    #[inline]
    pub fn has_leaf(&self) -> bool {
        self.0 >> 8 & 1 == 1
    }

    /// Returns true if the unit is a leaf which have a value. Otherwise, return false.
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.0 >> 31 == 1
    }

    /// Returns a 31 bits unsigned integer value associated with the unit.
    #[inline]
    pub fn value(&self) -> u32 {
        self.0 & 0x7FFFFFFF
    }

    /// Returns a label (<= 255) if the unit is not a leaf. Otherwise, returns an integer value greater
    /// than 255.
    #[inline]
    pub fn label(&self) -> u32 {
        self.0 & ((1 << 31) | 0xFF)
    }

    /// Returns an offset value within the unit. If the offset extension flag is true, returns the
    /// offset multiplied by 256.
    #[inline]
    pub fn offset(&self) -> u32 {
        (self.0 >> 10) << ((self.0 & (1 << 9)) >> 6)
    }

    /// Sets an offset to the unit. `offset` should be a value less than or equal to 29 bits. If the
    /// `offset` is greater than 21 bits, sets the offset extension flag and the `offset` without
    /// lower 8 bits (then, lower 8 bits of the given `offset` should be 0).
    #[inline]
    pub fn set_offset(&mut self, offset: u32) {
        assert!(offset < (1u32 << 29));

        if offset < (1u32 << 21) {
            // don't extend offset
            self.0 = offset << 10 | (self.0 << 23 as u32) >> 23;
        } else {
            // extend offset
            assert_eq!((offset << 2) & (1 << 31), 0, "MSB of offset should be 0");
            assert_eq!(offset & 0xFF, 0, "lower 8 bits of offset should be 0");
            self.0 = offset << 2 | (1 << 9) | (self.0 << 23 as u32) >> 23; // with offset extension flag
        }
    }

    /// Sets a `has_leaf` flag to the unit.
    #[inline]
    pub fn set_has_leaf(&mut self, has_leaf: bool) {
        self.0 = if has_leaf {
            self.0 | 1 << 8
        } else {
            self.0 & !(1 << 8)
        }
    }

    /// Sets a label to the unit.
    #[inline]
    pub fn set_label(&mut self, label: u8) {
        self.0 = (self.0 >> 8) << 8 | (label as u32)
    }

    /// Sets a value to the unit.
    #[inline]
    pub fn set_value(&mut self, value: u32) {
        self.0 = value | 1 << 31
    }

    /// Returns a string representation of the unit.
    pub fn to_string(&self) -> String {
        if self.is_leaf() {
            // leaf node
            format!("Unit {{ value: {} }}", self.value()).to_string()
        } else {
            // internal node
            let label = self.label();
            format!(
                "Unit {{ offset: {}, label: {}, has_leaf: {} }}",
                self.offset(),
                match label {
                    0 => "NULL".to_string(),
                    1..=255 => ((label as u8) as char).escape_default().to_string(),
                    _ => "INVALID".to_string(),
                },
                self.has_leaf()
            )
            .to_string()
        }
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::Unit;

    #[test]
    fn test_unit_value() {
        let mut unit = Unit::new();
        assert_eq!(unit.value(), 0);

        unit.set_value(5);
        assert_eq!(unit.value(), 5);

        unit.set_value((1 << 31) - 1);
        assert_eq!(unit.value(), (1 << 31) - 1);

        unit.set_value(1 << 31);
        assert_eq!(unit.value(), 0);
    }

    #[test]
    fn test_label() {
        let unit = Unit::new();
        assert_eq!(unit.label(), 0);

        let mut unit = Unit::new();
        unit.set_label(0);
        assert_eq!(unit.label(), 0);

        let mut unit = Unit::new();
        unit.set_label(1);
        assert_eq!(unit.label(), 1);

        let mut unit = Unit::new();
        unit.set_label(255);
        assert_eq!(unit.label(), 255);
    }

    #[test]
    fn test_offset() {
        let unit = Unit::new();
        assert_eq!(unit.offset(), 0);

        let mut unit = Unit::new();
        unit.set_offset(0);
        assert_eq!(unit.offset(), 0);

        let mut unit = Unit::new();
        unit.set_offset(1);
        assert_eq!(unit.offset(), 1);

        let mut unit = Unit::new();
        unit.set_offset((1 << 21) - 1);
        assert_eq!(unit.offset(), (1 << 21) - 1);

        let mut unit = Unit::new();
        unit.set_offset(1 << 21);
        assert_eq!(unit.offset(), 1 << 21);

        let mut unit = Unit::new();
        unit.set_offset(1 << 28);
        assert_eq!(unit.offset(), 1 << 28);
    }
}
