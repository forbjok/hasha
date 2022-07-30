use std::path::PathBuf;

use crate::{checksum_set::ChecksumSet, error::*, util};

pub fn diff(a: PathBuf, b: PathBuf) -> Result<(), CliError> {
    let checksums_a: ChecksumSet = {
        let file = util::open_file(a).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    let checksums_b: ChecksumSet = {
        let file = util::open_file(b).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    let diff = checksums_a.diff(&checksums_b);

    if diff.is_different() {
        diff.print();
    } else {
        println!("No differences found.");
    }

    Ok(())
}
