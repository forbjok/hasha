use std::path::PathBuf;

use crate::{error::*, hasher::ChecksumSet, util};

pub fn diff(a: PathBuf, b: PathBuf) -> Result<(), CliError> {
    let checksums_a: ChecksumSet = {
        let file = util::open_file(a).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    let checksums_b: ChecksumSet = {
        let file = util::open_file(b).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    let diff = checksums_a.diff(&checksums_b);

    if diff.is_different() {
        if !diff.additional_files.is_empty() {
            println!("-- ADDITIONAL FILES --");

            for p in diff.additional_files.iter() {
                println!("{}", p.display());
            }

            println!();
        }

        if !diff.missing_files.is_empty() {
            println!("-- MISSING FILES --");

            for p in diff.missing_files.iter() {
                println!("{}", p.display());
            }

            println!();
        }

        if !diff.differing_hashes.is_empty() {
            println!("-- DIFFERING HASHES --");

            for (p, (a, b)) in diff.differing_hashes.iter() {
                println!("{} == A: {} / B: {}", p.display(), a, b);
            }

            println!();
        }
    } else {
        println!("No differences found.");
    }

    Ok(())
}
