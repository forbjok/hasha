use std::{io, path::Path};

use sha2::{Digest, Sha256};

use crate::util;

pub fn hash_file<C: FnMut(usize)>(path: &Path, callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha256 = Sha256::new();

    let mut writer = progress_streams::ProgressWriter::new(&mut sha256, callback);
    io::copy(&mut file, &mut writer).unwrap();

    let hash = sha256.finalize();

    Ok(hex::encode(hash))
}
