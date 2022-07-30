use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use tracing::warn;

use crate::{ui::UiHandler, util};

use super::{ChecksumSet, HashType};

#[derive(Debug)]
pub struct ChecksumSetBuilder {
    hash_type: HashType,
    root_path: PathBuf,
    files: Vec<PathBuf>,
}

impl ChecksumSetBuilder {
    pub fn new(hash_type: HashType, root_path: &Path) -> Self {
        Self {
            hash_type,
            root_path: root_path.into(),
            files: Vec::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        let path = util::normalize_path(path);
        self.files.push(path);
    }

    pub fn add_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let entries = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok());

        for entry in entries {
            if !entry.file_type().is_file() {
                continue;
            }

            self.add_file(entry.path());
        }

        self
    }

    pub fn build(&self, ui: &mut dyn UiHandler) -> Result<ChecksumSet, anyhow::Error> {
        let hash_type = self.hash_type;
        let root_path = &self.root_path;

        let mut files: BTreeMap<String, String> = BTreeMap::new();

        let total_size: u64 = self
            .files
            .iter()
            .map(|path| std::fs::metadata(path).map(|m| m.len()).unwrap_or(0))
            .sum();

        ui.begin_generate(self.files.len() as u32, total_size);

        for path in self.files.iter() {
            // Make path relative, as we only want to match on the path
            // relative to the root.
            if let Ok(rel_path) = path.strip_prefix(&root_path) {
                let hash = hash_type.hash_file(path, ui)?;

                let rel_path = util::unixify_path(rel_path);
                files.insert(rel_path, hash);
            } else {
                warn!("'{}' is outside the root path. Skipping.", path.display());
            }
        }

        ui.end_generate();

        Ok(ChecksumSet { hash_type, files })
    }
}
