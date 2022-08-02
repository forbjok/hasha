use crate::ui::UiHandler;

use super::ChecksumSet;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct ChecksumSetDiff {
    pub additional_files: BTreeSet<String>,
    pub missing_files: BTreeSet<String>,
    pub differing_sizes: BTreeMap<String, (u64, u64)>,
    pub differing_hashes: BTreeMap<String, (String, String)>,
}

impl ChecksumSet {
    pub fn diff(&self, other: &ChecksumSet, ui: &mut dyn UiHandler) -> Result<ChecksumSetDiff, anyhow::Error> {
        if other.hash_type != self.hash_type {
            return Err(anyhow::anyhow!(
                "Checksum sets have mismatching hash types. Comparison makes no sense."
            ));
        }

        ui.begin_diff();

        let additional_files = self
            .files
            .keys()
            .filter(|p| !other.files.contains_key(*p))
            .cloned()
            .collect();

        let mut missing_files: BTreeSet<String> = BTreeSet::new();
        let mut differing_sizes: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        let mut differing_hashes: BTreeMap<String, (String, String)> = BTreeMap::new();

        for (path, other_fi) in other.files.iter() {
            if let Some(fi) = self.files.get(path) {
                if other_fi.size != fi.size {
                    differing_sizes.insert(path.into(), (fi.size, other_fi.size));
                } else if other_fi.hash != fi.hash {
                    differing_hashes.insert(path.into(), (fi.hash.clone(), other_fi.hash.clone()));
                }
            } else {
                missing_files.insert(path.into());
            }
        }

        ui.end_diff();

        Ok(ChecksumSetDiff {
            additional_files,
            missing_files,
            differing_sizes,
            differing_hashes,
        })
    }
}

impl ChecksumSetDiff {
    pub fn is_different(&self) -> bool {
        !self.additional_files.is_empty()
            || !self.missing_files.is_empty()
            || !self.differing_sizes.is_empty()
            || !self.differing_hashes.is_empty()
    }

    pub fn print(&self) {
        let mut summary: Vec<String> = Vec::new();

        println!();

        if !self.additional_files.is_empty() {
            println!("-- ADDITIONAL FILES --");

            for p in self.additional_files.iter() {
                println!("{}", p);
            }

            println!();

            summary.push(format!("{} additional files.", self.additional_files.len()));
        }

        if !self.missing_files.is_empty() {
            println!("-- MISSING FILES --");

            for p in self.missing_files.iter() {
                println!("{}", p);
            }

            println!();

            summary.push(format!("{} missing files.", self.missing_files.len()));
        }

        if !self.differing_sizes.is_empty() {
            println!("-- DIFFERING SIZES --");

            for (p, (a, b)) in self.differing_sizes.iter() {
                println!("{} == A: {} / B: {}", p, a, b);
            }

            println!();

            summary.push(format!("{} differing sizes.", self.differing_sizes.len()));
        }

        if !self.differing_hashes.is_empty() {
            println!("-- DIFFERING HASHES --");

            for (p, (a, b)) in self.differing_hashes.iter() {
                println!("{} == A: {} / B: {}", p, a, b);
            }

            println!();

            summary.push(format!("{} differing hashes.", self.differing_hashes.len()));
        }

        for line in summary.iter() {
            println!("{}", line);
        }
    }
}
