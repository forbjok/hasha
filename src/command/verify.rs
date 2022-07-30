use std::{fs, path::Path};

use anyhow::Context;

use crate::{checksum_set::ChecksumSet, ui::UiHandler};

pub fn verify(
    checksums_path: &Path,
    root_path: Option<&Path>,
    ui: &mut dyn UiHandler,
) -> Result<(), anyhow::Error> {
    let root_path = root_path.unwrap_or_else(|| checksums_path.parent().unwrap());

    let checksums: ChecksumSet = {
        let file = fs::File::open(&checksums_path)
            .with_context(|| format!("Opening checksum set file: {}", checksums_path.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set")?
    };

    let diff = checksums
        .verify(root_path, ui)
        .with_context(|| "Verifying files")?;

    if diff.is_different() {
        diff.print();
    } else {
        println!("Verified OK.");
    }

    Ok(())
}
