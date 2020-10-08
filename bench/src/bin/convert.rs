use itertools::Itertools;
use std::io;
use std::io::{BufRead, Error};

/// Output a TSV which contains byte-sorted keys and serial numbered values.
/// The keys are the first column in the given input CSV. The values will be generated.
///
/// # Usage
///
/// ```bash
/// $ cat input.csv | cargo run --bin convert > output.tsv
/// ```
///
/// or
///
/// ```bash
/// $ cargo build --release
/// $ cat input.csv | ../target/release/convert > output.tsv
/// ```
fn main() -> Result<(), Error> {
    let mut lexicon = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        lexicon.push(line.split(',').nth(0).unwrap().to_string());
    }

    // unique keys
    let mut keyset = lexicon
        .iter()
        // .map(|s| s.as_bytes())
        .unique()
        .collect::<Vec<_>>();

    // sort by byte-order
    keyset.sort();

    // number word IDs
    let keyset = keyset.into_iter().zip(1..).collect::<Vec<_>>();

    // output tsv to stdout
    for (key, value) in keyset {
        println!("{}\t{}", key, value);
    }

    Ok(())
}
