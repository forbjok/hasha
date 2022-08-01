use std::{fs, path::Path};

use anyhow::Context;

use crate::checksum_set::ChecksumSet;

pub fn diff(checksums_a_path: &Path, checksums_b_path: &Path) -> Result<(), anyhow::Error> {
    let checksums_a: ChecksumSet = {
        let file = fs::File::open(checksums_a_path)
            .with_context(|| format!("Opening checksum set A: {}", checksums_a_path.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set A")?
    };

    let checksums_b: ChecksumSet = {
        let file = fs::File::open(checksums_b_path)
            .with_context(|| format!("Opening checksum set B: {}", checksums_b_path.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set B")?
    };

    let diff = checksums_a.diff(&checksums_b)?;

    if diff.is_different() {
        diff.print();
    } else {
        println!("No differences found.");
    }

    Ok(())
}
