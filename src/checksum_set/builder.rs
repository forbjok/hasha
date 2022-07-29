use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use tracing::{info, warn};

use crate::util;

use super::{ChecksumSet, HashType};

#[derive(Debug)]
pub struct ChecksumSetBuilder {
    root_path: PathBuf,
    files: Vec<PathBuf>,
}

impl ChecksumSetBuilder {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            files: Vec::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        self.files.push(util::normalize_path(path));
    }

    pub fn build(self) -> ChecksumSet {
        let root_path = self.root_path;

        let mut files: BTreeMap<String, String> = BTreeMap::new();

        for path in self.files.into_iter() {
            // Make path relative, as we only want to match on the path
            // relative to the root.
            if let Ok(rel_path) = path.strip_prefix(&root_path) {
                let hash = util::hash_file(&path).unwrap();

                info!("HASH {} == {}", path.display(), hash);

                files.insert(util::unixify_path(rel_path), hash);
            } else {
                warn!("'{}' is outside the root path. Skipping.", path.display());
            }
        }

        ChecksumSet {
            hash_type: HashType::Sha256,
            files,
        }
    }
}
