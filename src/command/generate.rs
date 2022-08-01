use std::{fs, path::Path, time::Instant};

use anyhow::Context;

use crate::{
    checksum_set::{ChecksumSetBuilder, HashType},
    ui::UiHandler,
    util,
};

pub fn generate(
    path: &Path,
    output_file: Option<&Path>,
    root_path: Option<&Path>,
    hash_type: Option<HashType>,
    ui: &mut dyn UiHandler,
) -> Result<(), anyhow::Error> {
    let hash_type = hash_type.unwrap_or(HashType::Sha256);
    let path = util::normalize_path(path);

    let output_file = output_file.map(|p| p.to_path_buf()).unwrap_or_else(|| {
        path.file_name()
            .map(|n| path.with_file_name(format!("{}.checksums.json", n.to_string_lossy())))
            .unwrap_or_else(|| "checksums.json".into())
    });

    let root_path = root_path.unwrap_or_else(|| path.parent().unwrap_or(&path));

    let now = Instant::now();

    let checksum_set = ChecksumSetBuilder::new(hash_type, root_path)
        .add_path(&path, ui)
        .build(ui)
        .with_context(|| format!("Generating checksum set for path: {}", path.display()))?;

    eprintln!("Operation took {}.", util::humanize_duration(now.elapsed()));

    let file =
        fs::File::create(&output_file).with_context(|| format!("Creating output file: {}", output_file.display()))?;
    serde_json::to_writer_pretty(file, &checksum_set)
        .with_context(|| format!("Writing to output file: {}", output_file.display()))?;

    Ok(())
}
