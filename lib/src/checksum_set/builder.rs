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
    total_size: u64,
}

impl ChecksumSetBuilder {
    pub fn new(hash_type: HashType, root_path: &Path) -> Self {
        Self {
            hash_type,
            root_path: root_path.into(),
            files: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        let path = util::normalize_path(path);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        self.files.push(path);
        self.total_size += size;
    }

    pub fn add_path<P: AsRef<Path>>(&mut self, path: P, ui: &mut dyn UiHandler) -> &mut Self {
        ui.begin_scan();

        let entries = walkdir::WalkDir::new(path).into_iter().filter_map(|entry| entry.ok());

        for entry in entries {
            if !entry.file_type().is_file() {
                continue;
            }

            self.add_file(entry.path());
        }

        ui.end_scan();

        self
    }

    pub fn build(&self, ui: &mut dyn UiHandler) -> Result<ChecksumSet, anyhow::Error> {
        let hash_type = self.hash_type;
        let root_path = &self.root_path;

        let mut files: BTreeMap<String, String> = BTreeMap::new();

        ui.begin_generate(self.files.len() as u32, self.total_size);

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

        let checksums = ChecksumSet { hash_type, files };

        ui.end_generate();

        Ok(checksums)
    }
}
