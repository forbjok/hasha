use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::ui::UiHandler;

use super::{ChecksumSet, ChecksumSetDiff};

impl ChecksumSet {
    pub fn verify(&self, root_path: &Path, ui: &mut dyn UiHandler) -> Result<ChecksumSetDiff, anyhow::Error> {
        let hash_type = self.hash_type;

        ui.begin_prepare();

        let files: Vec<_> = self
            .files
            .iter()
            .map(|(path, hash)| {
                let actual_path = root_path.join(path);
                let size = std::fs::metadata(&actual_path).map(|m| m.len()).unwrap_or(0);

                (path, actual_path, size, hash)
            })
            .collect();

        let total_size: u64 = files.iter().map(|(_, _, size, _)| size).sum();

        ui.end_prepare();

        ui.begin_verify(files.len() as u32, total_size);

        let mut missing_files: BTreeSet<String> = Default::default();
        let mut differing_sizes: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        let mut differing_hashes: BTreeMap<String, (String, String)> = Default::default();

        for (path, actual_path, size, fi) in files.into_iter() {
            if !actual_path.exists() {
                missing_files.insert(path.to_string());
                continue;
            }

            if size != fi.size {
                differing_sizes.insert(path.to_string(), (fi.size, size));

                ui.file_progress(size);
                continue;
            }

            let hash = hash_type.hash_file(&actual_path, ui)?;

            if hash != fi.hash {
                differing_hashes.insert(path.to_string(), (fi.hash.clone(), hash));
            }
        }

        ui.end_verify();

        Ok(ChecksumSetDiff {
            additional_files: Default::default(),
            missing_files,
            differing_sizes,
            differing_hashes,
        })
    }
}
