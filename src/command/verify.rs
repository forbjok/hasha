use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::{checksum_set::ChecksumSet, ui::UiHandler};

pub fn verify(
    path: PathBuf,
    root_path: Option<PathBuf>,
    ui: &mut dyn UiHandler,
) -> Result<(), anyhow::Error> {
    let root_path = root_path.unwrap_or_else(|| path.parent().unwrap().into());

    let checksums: ChecksumSet = {
        let file = fs::File::open(&path)
            .with_context(|| format!("Opening checksum set file: {}", path.display()))?;
        serde_json::from_reader(file).with_context(|| "Deserializing checksum set")?
    };

    let diff = checksums
        .verify(&root_path, ui)
        .with_context(|| "Verifying files")?;

    if diff.is_different() {
        diff.print();
    } else {
        println!("Verified OK.");
    }

    Ok(())
}
