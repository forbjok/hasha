use std::collections::{BTreeMap, BTreeSet};

use super::ChecksumSet;

#[derive(Debug)]
pub struct ChecksumSetDiff {
    pub additional_files: BTreeSet<String>,
    pub missing_files: BTreeSet<String>,
    pub differing_hashes: BTreeMap<String, (String, String)>,
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
