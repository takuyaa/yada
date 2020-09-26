extern crate yada;

use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

fn main() {
    // make a keyset which have key-value pairs
    let keyset = &[
        ("a\0".as_bytes(), 0),
        ("ab\0".as_bytes(), 1),
        ("abc\0".as_bytes(), 2),
        ("b\0".as_bytes(), 3),
        ("bc\0".as_bytes(), 4),
        ("c\0".as_bytes(), 5),
    ];

    // build a double-array trie binary
    let da_bytes = DoubleArrayBuilder::build(keyset);
    assert!(da_bytes.is_some());

    // create a double-array trie instance
    let da = DoubleArray::new(da_bytes.unwrap());

    // exact match search
    for (key, value) in keyset {
        assert_eq!(da.exact_match_search(key), Some(*value as u32));
    }
    assert_eq!(da.exact_match_search("aa\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("aba\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("abb\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("abcd\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("ba\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("bb\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("bcd\0".as_bytes()), None);
    assert_eq!(da.exact_match_search("ca\0".as_bytes()), None);

    // common prefix search
    assert_eq!(
        da.common_prefix_search("a".as_bytes()).collect::<Vec<_>>(),
        vec![(0, 1)] //  match "a"
    );
    assert_eq!(
        da.common_prefix_search("abc".as_bytes())
            .collect::<Vec<_>>(),
        vec![(0, 1), (1, 2), (2, 3)] // match "a", "ab", "abc"
    );
    assert_eq!(
        da.common_prefix_search("abcd".as_bytes())
            .collect::<Vec<_>>(),
        vec![(0, 1), (1, 2), (2, 3)] // match "a", "ab", "abc"
    );
    assert_eq!(
        da.common_prefix_search("bcd".as_bytes())
            .collect::<Vec<_>>(),
        vec![(3, 1), (4, 2)] // match "b", "bc"
    );
    assert_eq!(
        da.common_prefix_search("d".as_bytes()).collect::<Vec<_>>(),
        vec![] // don't match
    );
}
