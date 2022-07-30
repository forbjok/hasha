use std::{io::Read, path::Path};

use byteorder::{BigEndian, ByteOrder};
use sha2::{Digest, Sha256};

use crate::util;

use super::HashType;

const BUFFER_SIZE: usize = 65536;

impl HashType {
    pub fn hash<C: FnMut(usize)>(
        &self,
        path: &Path,
        callback: C,
    ) -> Result<String, util::FileError> {
        match self {
            HashType::Crc32 => hash_crc32(path, callback),
            HashType::Sha256 => hash_sha256(path, callback),
        }
    }
}

fn hash_crc32<C: FnMut(usize)>(path: &Path, mut callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut hasher = crc32fast::Hasher::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        hasher.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash: u32 = hasher.finalize();

    let mut buf = [0u8; 4];
    BigEndian::write_u32(&mut buf, hash);

    Ok(hex::encode(buf))
}

fn hash_sha256<C: FnMut(usize)>(path: &Path, mut callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha256 = Sha256::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        sha256.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = sha256.finalize();

    Ok(hex::encode(hash))
}
