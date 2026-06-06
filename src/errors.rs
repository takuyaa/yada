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
        }
    }
}

impl std::error::Error for YadaError {}
