use std::{path::Path, time::Instant};

use anyhow::Context;

use hasha::{checksum_set::ChecksumSet, ui::UiHandler, util};

pub fn verify(checksums_path: &Path, root_path: Option<&Path>, ui: &mut dyn UiHandler) -> Result<(), anyhow::Error> {
    let root_path = root_path.unwrap_or_else(|| checksums_path.parent().unwrap());

    let checksums = ChecksumSet::load_from_file(checksums_path, ui)?;

    let now = Instant::now();

    let diff = checksums.verify(root_path, ui).with_context(|| "Verifying files")?;

    eprintln!("Operation took {}.", util::humanize_duration(now.elapsed()));

    if diff.is_different() {
        diff.print();
    } else {
        println!("Verified OK.");
    }

    Ok(())
}
