pub mod builder;
pub mod unit;

use crate::unit::{Unit, UnitID, UNIT_SIZE};
use std::convert::TryInto;
use std::ops::Deref;

/// A double array trie.
pub struct DoubleArray<T>(pub T)
where
    T: Deref<Target = [u8]>;

impl<T> DoubleArray<T>
where
    T: Deref<Target = [u8]>,
{
    /// Creates a new `DoubleArray` with a byte slice.
    pub fn new(bytes: T) -> Self {
        Self { 0: bytes }
    }

    /// Finds a value associated with a `key`.
    pub fn exact_match_search<K>(&self, key: K) -> Option<u32>
    where
        K: AsRef<[u8]>,
    {
        self.exact_match_search_bytes(key.as_ref())
    }

    fn exact_match_search_bytes(&self, key: &[u8]) -> Option<u32> {
        // traverse from root node
        let mut unit = self.get_unit(0 as UnitID)?;

        for &c in key.iter().take(key.len()) {
            assert!(!unit.is_leaf());
            assert_ne!(c, 0); // assumes characters don't have NULL ('\0')

            // try to traverse node
            let node_pos = (unit.offset() ^ (c as u32)) as UnitID;
            unit = self.get_unit(node_pos)?;

            if c != unit.label() as u8 {
                return None;
            }
        }

        if !unit.has_leaf() {
            return None;
        }

        // traverse node by NULL ('\0')
        let node_pos = (unit.offset() ^ (0u32)) as UnitID;
        unit = self.get_unit(node_pos)?;
        assert!(unit.is_leaf());
        assert!(unit.value() < (1 << 31));

        Some(unit.value())
    }

    /// Finds all values and it's key length which have a common prefix with a `key`.
    pub fn common_prefix_search<'b, K>(
        &'b self,
        key: &'b K,
    ) -> impl Iterator<Item = (u32, usize)> + 'b
    where
        K: AsRef<[u8]>,
        K: ?Sized,
    {
        self.common_prefix_search_bytes(key.as_ref())
    }

    fn common_prefix_search_bytes<'b>(
        &'b self,
        key: &'b [u8],
    ) -> impl Iterator<Item = (u32, usize)> + 'b {
        CommonPrefixSearch {
            key,
            double_array: self,
            unit_id: 0,
            key_pos: 0,
        }
    }

    fn get_unit(&self, index: usize) -> Option<Unit> {
        let b = &self.0[index * UNIT_SIZE..(index + 1) * UNIT_SIZE];
        match b.try_into() {
            Ok(bytes) => Some(Unit::from_u32(u32::from_le_bytes(bytes))),
            Err(_) => None,
        }
    }
}

/// An iterator that finds all values with a common prefix.
pub struct CommonPrefixSearch<'k, 'd, T>
where
    T: Deref<Target = [u8]>,
{
    key: &'k [u8],
    double_array: &'d DoubleArray<T>,
    unit_id: UnitID,
    key_pos: usize,
}

impl<T> Iterator for CommonPrefixSearch<'_, '_, T>
where
    T: Deref<Target = [u8]>,
{
    type Item = (u32, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.key_pos < self.key.len() {
            let unit = self.double_array.get_unit(self.unit_id)?;

            let c = *self.key.get(self.key_pos)?;
            self.key_pos += 1;

            self.unit_id = (unit.offset() ^ c as u32) as UnitID;
            let unit = self.double_array.get_unit(self.unit_id)?;
            if unit.label() != c as u32 {
                return None;
            }
            if unit.has_leaf() {
                let leaf_pos = unit.offset();
                let leaf_unit = self.double_array.get_unit(leaf_pos as UnitID)?;
                return Some((leaf_unit.value(), self.key_pos));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::DoubleArrayBuilder;
    use crate::DoubleArray;

    #[test]
    fn test_build_search() {
        let keyset = &[
            ("a".as_bytes(), 0),
            ("ab".as_bytes(), 1),
            ("aba".as_bytes(), 2),
            ("ac".as_bytes(), 3),
            ("acb".as_bytes(), 4),
            ("acc".as_bytes(), 5),
            ("ad".as_bytes(), 6),
            ("ba".as_bytes(), 7),
            ("bb".as_bytes(), 8),
            ("bc".as_bytes(), 9),
            ("c".as_bytes(), 10),
            ("caa".as_bytes(), 11),
        ];

        let da_bytes = DoubleArrayBuilder::build(keyset);
        assert!(da_bytes.is_some());

        let da = DoubleArray::new(da_bytes.unwrap());

        for (key, value) in keyset {
            assert_eq!(da.exact_match_search(key), Some(*value as u32));
        }
        assert_eq!(da.exact_match_search("aa".as_bytes()), None);
        assert_eq!(da.exact_match_search("abc".as_bytes()), None);
        assert_eq!(da.exact_match_search("b".as_bytes()), None);
        assert_eq!(da.exact_match_search("ca".as_bytes()), None);

        assert_eq!(
            da.common_prefix_search("a".as_bytes()).collect::<Vec<_>>(),
            vec![(0, 1)]
        );
        assert_eq!(
            da.common_prefix_search("aa".as_bytes()).collect::<Vec<_>>(),
            vec![(0, 1)]
        );
        assert_eq!(
            da.common_prefix_search("abbb".as_bytes())
                .collect::<Vec<_>>(),
            vec![(0, 1), (1, 2)]
        );
        assert_eq!(
            da.common_prefix_search("abaa".as_bytes())
                .collect::<Vec<_>>(),
            vec![(0, 1), (1, 2), (2, 3)]
        );
        assert_eq!(
            da.common_prefix_search("caa".as_bytes())
                .collect::<Vec<_>>(),
            vec![(10, 1), (11, 3)]
        );
        assert_eq!(
            da.common_prefix_search("d".as_bytes()).collect::<Vec<_>>(),
            vec![]
        );
    }
}
