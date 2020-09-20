use crate::unit::{Unit, UnitID};

const BLOCK_SIZE: usize = 256;
const NUM_TARGET_BLOCKS: i32 = 16; // the number of target blocks to find offsets
const INVALID_NEXT: u8 = 0; // 0 means that there is no next unused unit
const INVALID_PREV: u8 = 255; // 255 means that there is no previous unused unit

/// A double-array trie builder.
#[derive(Debug)]
pub struct DoubleArrayBuilder {
    pub blocks: Vec<DoubleArrayBlock>,
}

impl DoubleArrayBuilder {
    /// Constructs a new `DoubleArrayBuilder` with an empty `DoubleArrayBlock`.
    pub fn new() -> Self {
        Self {
            blocks: vec![DoubleArrayBlock::new()],
        }
    }

    /// Builds a double-array trie with a `keyset` and returns it when build finished successfully.
    /// Otherwise, returns `None`.
    /// The `keyset` must be sorted and each key must have a terminal character '\0'.
    pub fn build<'a, T>(keyset: &[(T, u32)]) -> Option<Vec<u8>>
    where
        T: AsRef<[u8]>,
    {
        Self::new().build_from_keyset(keyset)
    }

    /// Builds a double-array trie with a `keyset` and returns it when build finished successfully.
    /// Otherwise, returns `None`.
    /// The `keyset` must be sorted and each key must have a terminal character '\0'.
    pub fn build_from_keyset<T>(&mut self, keyset: &[(T, u32)]) -> Option<Vec<u8>>
    where
        T: AsRef<[u8]>,
    {
        self.reserve(0); // reserve root node
        self.build_recursive(keyset, 0, 0, keyset.len(), 0)?;

        let mut da_bytes = Vec::with_capacity(self.blocks.len() * BLOCK_SIZE);
        for block in &self.blocks {
            for unit in block.units.iter() {
                let bytes = unit.as_u32().to_le_bytes();
                da_bytes.extend_from_slice(&bytes);
            }
        }

        Some(da_bytes)
    }

    /// Returns the number of `Unit`s that this builder contains.
    pub fn num_units(&self) -> u32 {
        (self.blocks.len() * BLOCK_SIZE) as u32
    }

    /// Returns the number of used `Unit`s that this builder contains.
    pub fn num_used_units(&self) -> u32 {
        self.blocks
            .iter()
            .map(|block| {
                block
                    .is_used
                    .iter()
                    .fold(0, |acc, &is_used| acc + if is_used { 1 } else { 0 })
            })
            .sum::<u32>()
    }

    fn get_block(&self, unit_id: UnitID) -> Option<&DoubleArrayBlock> {
        self.blocks.get(unit_id / BLOCK_SIZE)
    }

    fn get_block_mut(&mut self, unit_id: UnitID) -> Option<&mut DoubleArrayBlock> {
        self.blocks.get_mut(unit_id / BLOCK_SIZE)
    }

    fn extend_block_mut(&mut self) -> &mut DoubleArrayBlock {
        self.blocks.push(DoubleArrayBlock::new());
        self.blocks.last_mut().unwrap()
    }

    fn get_unit_mut(&mut self, unit_id: UnitID) -> &mut Unit {
        while self.get_block(unit_id).is_none() {
            self.extend_block_mut();
        }
        let block = self.get_block_mut(unit_id).unwrap();
        &mut block.units[unit_id % BLOCK_SIZE]
    }

    fn reserve(&mut self, unit_id: UnitID) {
        while self.get_block(unit_id).is_none() {
            self.extend_block_mut();
        }
        let block = self.get_block_mut(unit_id).unwrap();
        assert!(unit_id % BLOCK_SIZE < 256);
        block.reserve((unit_id % BLOCK_SIZE) as u8);
    }

    fn build_recursive<T>(
        &mut self,
        keyset: &[(T, u32)],
        depth: usize,
        begin: usize,
        end: usize,
        unit_id: UnitID,
    ) -> Option<()>
    where
        T: AsRef<[u8]>,
    {
        // element of labels is a tuple (label, start_position, end_position)
        let mut labels: Vec<(u8, usize, usize)> = Vec::with_capacity(256);
        let mut value = None;

        for i in begin..end {
            let key_value = keyset.get(i)?;
            let label = *key_value.0.as_ref().get(depth)?;
            if label == 0 {
                assert!(value.is_none()); // there is just one '\0' in a key
                value = Some(key_value.1);
            }
            match labels.last_mut() {
                Some(last_label) => {
                    if last_label.0 != label {
                        last_label.2 = i; // set end position
                        labels.push((label, i, 0));
                    }
                }
                None => {
                    labels.push((label, i, 0));
                }
            }
        }
        assert!(labels.len() > 0);

        let mut last_label = labels.last_mut().unwrap();
        last_label.2 = end;

        let labels_ = labels.iter().map(|(key, _, _)| *key).collect::<Vec<_>>();
        assert!(labels_.len() > 0);

        // search an offset where these children fits to unused positions.
        let offset = self.find_offset(&labels_);
        assert!(offset < 16_777_216); // offset must be represented as 23 bits integer

        let has_leaf = labels_.first().filter(|&&x| x == 0).is_some();

        // populate offset and has_leaf flag to parent node
        let parent_unit = self.get_unit_mut(unit_id);
        // upper 24 bits should be 0
        assert_eq!(parent_unit.offset(), 0);
        assert!(!parent_unit.has_leaf());
        parent_unit.set_offset(offset);
        parent_unit.set_has_leaf(has_leaf);

        // populate label or associated value to children node
        for label in labels_ {
            let child_id = (offset ^ label as u32) as UnitID;
            self.reserve(child_id);

            let unit = self.get_unit_mut(child_id);

            // child node units should be empty
            assert_eq!(unit.offset(), 0);
            assert_eq!(unit.label(), 0);
            assert_eq!(unit.value(), 0);
            assert!(!unit.has_leaf());

            if label == 0 {
                assert!(value.is_some());
                unit.set_value(value.unwrap());
            } else {
                unit.set_label(label);
            }
        }

        // recursive call in depth-first order
        for (label, begin, end) in labels {
            self.build_recursive(
                keyset,
                depth + 1,
                begin,
                end,
                (label as u32 ^ offset) as UnitID,
            );
        }

        Some(())
    }

    fn find_offset(&mut self, labels: &Vec<u8>) -> u32 {
        let head_block = (self.blocks.len() as i32 - NUM_TARGET_BLOCKS).max(0) as usize;
        let offset = self
            .blocks
            .iter_mut()
            .enumerate()
            .skip(head_block) // search for offset in last N blocks
            .find_map(|(block_id, block)| {
                block.find_offset(labels).map(|offset| (block_id, offset))
            });
        let (block_id, offset) = offset.unwrap_or_else(|| {
            let new_block = self.extend_block_mut();

            // offset should be always found in an empty block
            let offset = new_block.find_offset(labels).unwrap();
            let block_id = self.blocks.len() - 1;

            (block_id, offset)
        });

        (block_id as u32) << 8 | offset as u32
    }
}

const DEFAULT_UNITS: [Unit; BLOCK_SIZE] = [Unit::new(); BLOCK_SIZE];
const DEFAULT_IS_USED: [bool; BLOCK_SIZE] = [false; BLOCK_SIZE];
const DEFAULT_NEXT_UNUSED: [u8; BLOCK_SIZE] = {
    let mut next_unused = [INVALID_NEXT; BLOCK_SIZE];
    let mut i = 0;
    while i < next_unused.len() - 1 {
        next_unused[i] = (i + 1) as u8;
        i += 1;
    }
    next_unused
};
const DEFAULT_PREV_UNUSED: [u8; BLOCK_SIZE] = {
    let mut prev_unused = [INVALID_PREV; BLOCK_SIZE];
    let mut i = 1;
    while i < prev_unused.len() {
        prev_unused[i] = (i - 1) as u8;
        i += 1;
    }
    prev_unused
};

/// A block that have a shard of a double-array and other useful data structures.
pub struct DoubleArrayBlock {
    pub units: [Unit; BLOCK_SIZE],
    pub is_used: [bool; BLOCK_SIZE],
    pub head_unused: u8,
    pub next_unused: [u8; BLOCK_SIZE],
    pub prev_unused: [u8; BLOCK_SIZE],
}

impl DoubleArrayBlock {
    const fn new() -> Self {
        Self {
            units: DEFAULT_UNITS,
            is_used: DEFAULT_IS_USED,
            head_unused: 0,
            next_unused: DEFAULT_NEXT_UNUSED,
            prev_unused: DEFAULT_PREV_UNUSED,
        }
    }

    /// Finds a valid offset in this block.
    fn find_offset(&mut self, labels: &Vec<u8>) -> Option<u8> {
        assert!(labels.len() > 0);

        // return if this block is full
        if self.head_unused == INVALID_NEXT && self.is_used[0] == true {
            assert!(self.is_used.iter().all(|is_used| *is_used)); // assert full
            return None;
        }
        assert!(!self.is_used.iter().all(|is_used| *is_used)); // assert not full

        let mut unused_id = self.head_unused;
        loop {
            assert!(!self.is_used[unused_id as usize]);

            let first_label = *labels.first()?;
            let offset = unused_id ^ first_label;
            let all_unused = labels.iter().skip(1).all(|label| {
                let id = offset ^ label;
                match self.is_used.get(id as UnitID) {
                    Some(is_used) => !*is_used,
                    None => {
                        // something is going wrong
                        assert!(false);
                        false
                    }
                }
            });
            if all_unused {
                return Some(offset);
            }

            // try next unused_id
            unused_id = self.next_unused[unused_id as usize];

            if unused_id == INVALID_NEXT {
                return None;
            }
        }
    }

    fn reserve(&mut self, id: u8) {
        // maintain is_used
        self.is_used[id as usize] = true;

        let prev_id = self.prev_unused[id as usize];
        let next_id = self.next_unused[id as usize];

        // maintain next_unused
        if prev_id != INVALID_PREV {
            self.next_unused[prev_id as usize] = next_id;
        }
        self.next_unused[id as usize] = INVALID_NEXT; // this line can be removed

        // maintain prev_unused
        if next_id != INVALID_NEXT {
            self.prev_unused[next_id as usize] = prev_id;
        }
        self.prev_unused[id as usize] = INVALID_PREV; // this line can be removed

        // maintain head_unused
        if id == self.head_unused {
            self.head_unused = next_id;
        }
    }
}

impl std::fmt::Debug for DoubleArrayBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DoubleArrayBlock")
            .field(
                "units",
                &format_args!(
                    "[{}]",
                    self.units
                        .iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .field(
                "is_used",
                &format_args!(
                    "[{}]",
                    self.is_used
                        .iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .field("head_unused", &self.head_unused)
            .field(
                "next_unused",
                &format_args!(
                    "[{}]",
                    self.next_unused
                        .iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .field(
                "prev_unused",
                &format_args!(
                    "[{}]",
                    self.prev_unused
                        .iter()
                        .map(|u| u.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::DoubleArrayBuilder;

    #[test]
    fn test_build() {
        let keyset: &[(&[u8], u32)] = &[
            ("a\0".as_bytes(), 0),
            ("aa\0".as_bytes(), 0),
            ("aaa\0".as_bytes(), 0),
            ("aaaa\0".as_bytes(), 0),
            ("aaaaa\0".as_bytes(), 0),
            ("ab\0".as_bytes(), 0),
            ("abc\0".as_bytes(), 0),
            ("abcd\0".as_bytes(), 0),
            ("abcde\0".as_bytes(), 0),
            ("abcdef\0".as_bytes(), 0),
        ];

        let mut builder = DoubleArrayBuilder::new();
        let da = builder.build_from_keyset(keyset);
        assert!(da.is_some());

        assert!(0 < builder.num_units());
        assert!(0 < builder.num_used_units());
        assert!(builder.num_used_units() < builder.num_units());
    }
}
