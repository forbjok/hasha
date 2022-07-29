use std::path::PathBuf;

use crate::{error::*, hasher::ChecksumSetBuilder, util};

pub fn calculate(
    files: Vec<String>,
    output_file: Option<PathBuf>,
    root_path: Option<PathBuf>,
) -> Result<(), CliError> {
    let output_file = output_file.unwrap_or_else(|| "checksums.json".into());
    let root_path = root_path.unwrap_or_else(|| std::env::current_dir().unwrap());

    // Transform list of glob patterns into a list of actual file paths
    let file_paths = files
        .into_iter()
        .filter_map(|pattern| glob::glob(&pattern).ok())
        .flatten()
        .filter_map(|path| path.ok());

    let mut builder = ChecksumSetBuilder::new(root_path);

    for file_path in file_paths {
        if !file_path.is_file() {
            continue;
        }

        builder.add_file(file_path);
    }

    let checksum_set = builder.build();

    let file = util::create_file(output_file).unwrap();
    serde_json::to_writer_pretty(file, &checksum_set).unwrap();

    Ok(())
}
