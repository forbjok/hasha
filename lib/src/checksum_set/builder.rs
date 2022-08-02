use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use tracing::warn;

use crate::{ui::UiHandler, util};

use super::{ChecksumSet, FileInfo, HashType};

#[derive(Debug)]
struct BuilderFileInfo {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub struct ChecksumSetBuilder {
    hash_type: HashType,
    root_path: PathBuf,
    files: Vec<BuilderFileInfo>,
}

impl ChecksumSetBuilder {
    pub fn new(hash_type: HashType, root_path: &Path) -> Self {
        Self {
            hash_type,
            root_path: util::normalize_path(root_path),
            files: Vec::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        let path = util::normalize_path(path);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        self.files.push(BuilderFileInfo { path, size });
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

        let mut files: BTreeMap<String, FileInfo> = BTreeMap::new();

        ui.begin_prepare();

        let total_size: u64 = self
            .files
            .iter()
            .map(|fi| std::fs::metadata(&fi.path).map(|m| m.len()).unwrap_or(0))
            .sum();

        ui.end_prepare();

        ui.begin_generate(self.files.len() as u32, total_size);

        for BuilderFileInfo { path, size } in self.files.iter() {
            // Make path relative, as we only want to match on the path
            // relative to the root.
            if let Ok(rel_path) = path.strip_prefix(&root_path) {
                let hash = hash_type.hash_file(path, ui)?;

                let rel_path = util::unixify_path(rel_path);
                let size = *size;

                files.insert(rel_path, FileInfo { hash, size });
            } else {
                warn!("'{}' is outside the root path. Skipping.", path.display());
            }
        }

        let checksums = ChecksumSet { hash_type, files };

        ui.end_generate();

        Ok(checksums)
    }
}
