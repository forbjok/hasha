use std::path::PathBuf;

use crate::{checksum_set::ChecksumSetBuilder, error::*, util};

pub fn generate(
    path: PathBuf,
    output_file: Option<PathBuf>,
    root_path: Option<PathBuf>,
) -> Result<(), CliError> {
    let path = util::normalize_path(path);

    let output_file =
        output_file.unwrap_or_else(|| format!("{}.checksums.json", path.display()).into());

    let root_path = root_path.unwrap_or_else(|| path.parent().unwrap_or(&path).into());

    let entries = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok());

    let mut builder = ChecksumSetBuilder::new(root_path);

    for entry in entries {
        if !entry.file_type().is_file() {
            continue;
        }

        builder.add_file(entry.path());
    }

    let checksum_set = builder.build();

    let file = util::create_file(output_file).unwrap();
    serde_json::to_writer_pretty(file, &checksum_set).unwrap();

    Ok(())
}
