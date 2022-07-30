use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::checksum_set::ChecksumSet;

pub fn diff(a: PathBuf, b: PathBuf) -> Result<(), anyhow::Error> {
    let checksums_a: ChecksumSet = {
        let file = fs::File::open(&a)
            .with_context(|| format!("Opening checksum set A: {}", a.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set A")?
    };

    let checksums_b: ChecksumSet = {
        let file = fs::File::open(&b)
            .with_context(|| format!("Opening checksum set B: {}", b.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set B")?
    };

    let diff = checksums_a.diff(&checksums_b);

    if diff.is_different() {
        diff.print();
    } else {
        println!("No differences found.");
    }

    Ok(())
}
