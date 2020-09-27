# Yada: Yet Another Double-Array

[![crate-name at crates.io](https://img.shields.io/crates/v/yada.svg)](https://crates.io/crates/yada)
[![crate-name at docs.rs](https://docs.rs/yada/badge.svg)](https://docs.rs/yada)

Yada is a yet another double-array trie library aiming for fast search and
compact data representation.

## Features

- Build static double-array tries
  - Yada adopts the compact binary representation of double-array nodes like
  [Darts-clone](https://github.com/s-yata/darts-clone).
- Common prefix search
  - The method returns an `Iterator` that is an effective way to find multiple
  values without heap allocation.
- Exact match search
  - The method finds a value associated with an exact match key as a `Option`.

## Requirements

- Rust version >= 1.46.0 

## Usage

See also [example code](examples/build_and_search.rs) for more details.

### Build a double-array trie

```rust
use yada::builder::DoubleArrayBuilder;

// make a keyset which have key-value pairs
let keyset = &[
    ("a".as_bytes(), 0),
    ("ab".as_bytes(), 1),
    ("abc".as_bytes(), 2),
    ("b".as_bytes(), 3),
    ("bc".as_bytes(), 4),
    ("c".as_bytes(), 5),
];

// build a double-array trie binary
let da_bytes: Option<Vec<u8>> = DoubleArrayBuilder::build(keyset);
```

### Search entries by keys

```rust
use yada::DoubleArray;

// create a double-array trie instance
let da = DoubleArray::new(da_bytes.unwrap());

// exact match search
for (key, value) in keyset {
    assert_eq!(da.exact_match_search(key), Some(*value as u32));
}
assert_eq!(da.exact_match_search("abc".as_bytes()), Some(2));
assert_eq!(da.exact_match_search("abcd".as_bytes()), None);

// common prefix search
assert_eq!(
    da.common_prefix_search("abcd".as_bytes())
        .collect::<Vec<_>>(),
    vec![(0, 1), (1, 2), (2, 3)] // match "a", "ab", "abc", value and key length
);
assert_eq!(
    da.common_prefix_search("d".as_bytes()).collect::<Vec<_>>(),
    vec![] // don't match
);
```

## Limitations

- The value must be represented as a 31 bit unsigned integer, typed `u32`.
  - Yada uses the most significant bit (MSB) as a flag to distinguish between a value node and others.
- The offset of an double-array node is 23 bit wide, so it can only represent up to
 ~8.3M nodes.
  - It means this limitation results in the size upper bound ~33 MB of double-arrays.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## References

- [Aoe, J. An Efficient Digital Search Algorithm by Using a Double-Array Structure.
IEEE Transactions on Software Engineering. Vol. 15, 9 (Sep 1989). pp. 1066-1077.](https://ieeexplore.ieee.org/document/31365)
- [Darts: Double ARray Trie System](http://chasen.org/~taku/software/darts/)
- [Darts-clone: A clone of Darts (Double-ARray Trie System)](https://github.com/s-yata/darts-clone)
