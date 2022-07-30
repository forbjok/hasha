use std::path::Path;

use sha2::{Digest, Sha256};

use crate::util;

use super::HashType;

impl HashType {
    pub fn hash<C: FnMut(usize)>(
        &self,
        path: &Path,
        callback: C,
    ) -> Result<String, util::FileError> {
        match self {
            HashType::Sha256 => hash_sha256(path, callback),
        }
    }
}

fn hash_sha256<C: FnMut(usize)>(path: &Path, callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha256 = Sha256::new();

    let mut writer = progress_streams::ProgressWriter::new(&mut sha256, callback);
    std::io::copy(&mut file, &mut writer).unwrap();

    let hash = sha256.finalize();

    Ok(hex::encode(hash))
}
