use std::path::Path;

use anyhow::Context;

use crate::ui::UiHandler;

use super::ChecksumSet;

impl ChecksumSet {
    pub fn load_from_file(path: &Path, ui: &mut dyn UiHandler) -> Result<Self, anyhow::Error> {
        let filename = path.file_name().unwrap().to_string_lossy();

        ui.begin_load(&filename);

        let checksums = {
            let file =
                std::fs::File::open(path).with_context(|| format!("Opening checksum set file: {}", path.display()))?;

            serde_json::from_reader(file).with_context(|| "Deserializing checksum set")?
        };

        ui.end_load();

        Ok(checksums)
    }
}
