//! Definition of errors.
use std::{fmt, result};

/// A specialized `Result` type for Yada.
pub type Result<T, E = YadaError> = result::Result<T, E>;

/// Errors in Yada.
#[derive(Debug, Eq, PartialEq)]
pub enum YadaError {
    /// The keyset is empty.
    EmptyKeyset,

    /// The keyset contains an empty key.
    EmptyKey,

    /// A key contains a null byte.
    NullByte,

    /// The keyset contains duplicated keys.
    DuplicateKey,

    /// The keyset is not sorted.
    UnsortedKeyset,

    /// An input value exceeds the maximum value.
    ValueTooLarge { max: u32 },

    /// The resulting trie exceeds the maximum number of units.
    TooManyUnits { max: u32 },

    /// The input double-array trie is empty.
    EmptyDoubleArray,

    /// The input double-array trie byte length is not aligned to the unit size.
    UnalignedDoubleArray { len: usize, unit_size: usize },

    /// A unit in the input double-array trie refers to a child unit outside the input.
    InvalidDoubleArrayUnit {
        index: usize,
        child_index: usize,
        num_units: usize,
    },
}

impl fmt::Display for YadaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyKeyset => write!(f, "keyset must not be empty"),
            Self::EmptyKey => write!(f, "keyset must not contain an empty key"),
            Self::NullByte => write!(f, "keys must not contain NULL"),
            Self::DuplicateKey => write!(f, "keyset must not contain duplicated keys"),
            Self::UnsortedKeyset => write!(f, "keyset must be sorted"),
            Self::ValueTooLarge { max } => {
                write!(f, "input value must be no greater than {max}")
            }
            Self::TooManyUnits { max } => {
                write!(f, "num_units must be no greater than {max}")
            }
            Self::EmptyDoubleArray => write!(f, "double-array trie must not be empty"),
            Self::UnalignedDoubleArray { len, unit_size } => write!(
                f,
                "double-array trie byte length ({len}) must be a multiple of unit size ({unit_size})"
            ),
            Self::InvalidDoubleArrayUnit {
                index,
                child_index,
                num_units,
            } => write!(
                f,
                "double-array trie unit at index {index} can refer to child index {child_index}, but num_units is {num_units}"
            ),
        }
    }
}

impl std::error::Error for YadaError {}
