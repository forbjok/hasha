use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use tracing::warn;

use crate::{ui::UiHandler, util};

use super::{ChecksumSet, HashType};

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    size: u64,
}

#[derive(Debug)]
pub struct ChecksumSetBuilder {
    root_path: PathBuf,
    files: Vec<FileInfo>,
}

impl FileInfo {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let path = util::normalize_path(path);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        Self { path, size }
    }
}

impl ChecksumSetBuilder {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            files: Vec::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        self.files.push(FileInfo::from(path));
    }

    pub fn build(self, ui: &mut dyn UiHandler) -> ChecksumSet {
        let root_path = self.root_path;

        let mut files: BTreeMap<String, String> = BTreeMap::new();

        let total_size: u64 = self.files.iter().map(|fi| fi.size).sum();

        ui.begin_checksums(self.files.len() as u32, total_size);

        for FileInfo { path, size } in self.files.into_iter() {
            // Make path relative, as we only want to match on the path
            // relative to the root.
            if let Ok(rel_path) = path.strip_prefix(&root_path) {
                let rel_path = util::unixify_path(rel_path);

                ui.begin_file(&rel_path, size);
                let hash = util::hash_file(&path, |b| ui.file_progress(b as u64)).unwrap();
                ui.end_file();

                files.insert(rel_path, hash);
            } else {
                warn!("'{}' is outside the root path. Skipping.", path.display());
            }
        }

        ui.end_checksums();

        ChecksumSet {
            hash_type: HashType::Sha256,
            files,
        }
    }
}
