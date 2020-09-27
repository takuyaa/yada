extern crate yada;

use std::fs::File;
use std::io::{Read, Write};
use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // filename to save and load
    let filename = "load_from_file_example.da";

    // make a keyset which have key-value pairs
    let keyset = &[
        ("a".as_bytes(), 0),
        ("aa".as_bytes(), 1),
        ("aaa".as_bytes(), 2),
        ("b".as_bytes(), 3),
        ("bcd".as_bytes(), 4),
    ];

    // build a double-array trie binary
    let da_bytes = DoubleArrayBuilder::build(keyset);
    assert!(da_bytes.is_some());

    // create a double-array trie instance
    let da = DoubleArray::new(da_bytes.unwrap());

    // save to file
    let mut file = File::create(filename)?;
    file.write_all(da.0.as_slice())?;
    file.flush()?;

    // load from file
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    let da = DoubleArray::new(buf);

    // test search
    for (key, value) in keyset.iter() {
        let v = da.exact_match_search(key).unwrap();
        assert_eq!(v, *value);
    }
    assert_eq!(
        da.common_prefix_search(&"aaaa").collect::<Vec<_>>(),
        vec![(0, 1), (1, 2), (2, 3)]
    );
    assert_eq!(
        da.common_prefix_search(&"bcde").collect::<Vec<_>>(),
        vec![(3, 1), (4, 3)]
    );

    Ok(())
}
