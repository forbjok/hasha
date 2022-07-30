use std::path::PathBuf;

use crate::{
    checksum_set::{ChecksumSetBuilder, HashType},
    error::*,
    ui::UiHandler,
    util,
};

pub fn generate(
    path: PathBuf,
    output_file: Option<PathBuf>,
    root_path: Option<PathBuf>,
    hash_type: Option<HashType>,
    ui: &mut dyn UiHandler,
) -> Result<(), CliError> {
    let hash_type = hash_type.unwrap_or(HashType::Sha256);
    let path = util::normalize_path(path);

    let output_file = output_file.unwrap_or_else(|| {
        path.file_name()
            .map(|n| path.with_file_name(format!("{}.checksums.json", n.to_string_lossy())))
            .unwrap_or_else(|| "checksums.json".into())
    });

    let root_path = root_path.unwrap_or_else(|| path.parent().unwrap_or(&path).into());

    let entries = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok());

    let mut builder = ChecksumSetBuilder::new(hash_type, root_path);

    for entry in entries {
        if !entry.file_type().is_file() {
            continue;
        }

        builder.add_file(entry.path());
    }

    let checksum_set = builder.build(ui);

    let file = util::create_file(output_file).unwrap();
    serde_json::to_writer_pretty(file, &checksum_set).unwrap();

    Ok(())
}
