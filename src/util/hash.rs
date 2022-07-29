use std::{io, path::Path};

use sha2::{Digest, Sha256};

use crate::util;

pub fn hash_file(path: &Path) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256).unwrap();

    let hash = sha256.finalize();

    Ok(hex::encode(hash))
}
