use std::{borrow::Cow, path::Path, time::Instant};

use anyhow::Context;

use kecs::{
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

    let output_file: Cow<Path> = output_file
        .map(Cow::Borrowed)
        .or_else(|| {
            path.file_name()
                .map(|n| Cow::Owned(path.with_file_name(format!("{}.kecs.json", n.to_string_lossy()))))
        })
        .with_context(|| "Could not determine output file path.")?;

    let root_path = root_path.or_else(|| path.parent()).unwrap_or(&path);

    let now = Instant::now();

    let checksum_set = ChecksumSetBuilder::new(hash_type, root_path)
        .add_path(&path, ui)
        .build(ui)
        .with_context(|| format!("Generating checksum set for path: {}", path.display()))?;

    eprintln!("Operation took {}.", util::humanize_duration(now.elapsed()));

    checksum_set.write_file(&output_file)?;

    Ok(())
}
