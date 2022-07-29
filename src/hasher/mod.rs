use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::util;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HashType {
    Sha256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChecksumSet {
    pub hash_type: HashType,
    pub files: BTreeMap<String, String>,
}

#[derive(Debug)]
pub struct ChecksumSetBuilder {
    root_path: PathBuf,
    files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct ChecksumSetDiff {
    pub additional_files: BTreeSet<String>,
    pub missing_files: BTreeSet<String>,
    pub differing_hashes: BTreeMap<String, (String, String)>,
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

impl ChecksumSet {
    pub fn diff(&self, other: &ChecksumSet) -> ChecksumSetDiff {
        let additional_files = self
            .files
            .keys()
            .filter(|p| !other.files.contains_key(*p))
            .cloned()
            .collect();

        let mut missing_files: BTreeSet<String> = BTreeSet::new();
        let mut differing_hashes: BTreeMap<String, (String, String)> = BTreeMap::new();

        for (path, other_hash) in other.files.iter() {
            if let Some(hash) = self.files.get(path) {
                if other_hash != hash {
                    differing_hashes.insert(path.into(), (hash.clone(), other_hash.clone()));
                }
            } else {
                missing_files.insert(path.into());
            }
        }

        ChecksumSetDiff {
            additional_files,
            missing_files,
            differing_hashes,
        }
    }
}

impl ChecksumSetDiff {
    pub fn is_different(&self) -> bool {
        !self.additional_files.is_empty()
            || !self.missing_files.is_empty()
            || !self.differing_hashes.is_empty()
    }
}
