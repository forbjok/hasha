use std::path::Path;

use crate::{checksum_set::ChecksumSet, ui::UiHandler};

pub fn diff(checksums_a_path: &Path, checksums_b_path: &Path, ui: &mut dyn UiHandler) -> Result<(), anyhow::Error> {
    let checksums_a = ChecksumSet::load_from_file(checksums_a_path, ui)?;
    let checksums_b = ChecksumSet::load_from_file(checksums_b_path, ui)?;

    let diff = checksums_a.diff(&checksums_b, ui)?;

    if diff.is_different() {
        diff.print();
    } else {
        println!("No differences found.");
    }

    Ok(())
}
