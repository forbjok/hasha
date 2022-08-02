use std::{fs, path::Path};

use anyhow::Context;

use super::ChecksumSet;

impl ChecksumSet {
    pub fn write_file(&self, path: &Path) -> Result<(), anyhow::Error> {
        let file = fs::File::create(path).with_context(|| format!("Creating output file: {}", path.display()))?;

        serde_json::to_writer_pretty(file, self)
            .with_context(|| format!("Writing to output file: {}", path.display()))?;

        Ok(())
    }
}
