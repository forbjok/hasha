use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::ui::UiHandler;

use super::{ChecksumSet, ChecksumSetDiff};

impl ChecksumSet {
    pub fn verify(&self, root_path: &Path, ui: &mut dyn UiHandler) -> ChecksumSetDiff {
        let hash_type = self.hash_type;

        let files: Vec<_> = self
            .files
            .iter()
            .map(|(path, hash)| {
                let actual_path = root_path.join(path);

                (path, actual_path, hash)
            })
            .collect();

        let total_size: u64 = files
            .iter()
            .map(|(_, path, _)| std::fs::metadata(path).map(|m| m.len()).unwrap_or(0))
            .sum();

        ui.begin_verify(files.len() as u32, total_size);

        let mut missing_files: BTreeSet<String> = Default::default();
        let mut differing_hashes: BTreeMap<String, (String, String)> = Default::default();

        for (path, actual_path, expected_hash) in files.into_iter() {
            if !actual_path.is_file() {
                missing_files.insert(path.to_string());
                continue;
            }

            let hash = hash_type.hash_file(&actual_path, ui);

            if &hash != expected_hash {
                differing_hashes.insert(path.to_string(), (expected_hash.to_string(), hash));
            }
        }

        ui.end_verify();

        ChecksumSetDiff {
            additional_files: Default::default(),
            missing_files,
            differing_hashes,
        }
    }
}
