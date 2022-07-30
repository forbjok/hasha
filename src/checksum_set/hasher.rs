use std::{io::Read, path::Path};

use blake2::{Blake2b512, Blake2s256};
use byteorder::{BigEndian, ByteOrder};
use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use sha3::Sha3_256;

use crate::{ui::UiHandler, util};

use super::HashType;

const BUFFER_SIZE: usize = 524288;

impl HashType {
    pub fn hash<C: FnMut(usize)>(
        &self,
        path: &Path,
        callback: C,
    ) -> Result<String, util::FileError> {
        match self {
            HashType::Blake2b512 => hash_blake2b512(path, callback),
            HashType::Blake2s256 => hash_blake2s256(path, callback),
            HashType::Crc32 => hash_crc32(path, callback),
            HashType::Md5 => hash_md5(path, callback),
            HashType::Sha1 => hash_sha1(path, callback),
            HashType::Sha256 => hash_sha256(path, callback),
            HashType::Sha3_256 => hash_sha3_256(path, callback),
        }
    }

    pub fn hash_file(&self, path: &Path, ui: &mut dyn UiHandler) -> String {
        let filename = path.file_name().unwrap().to_string_lossy();
        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

        ui.begin_file(&filename, size);

        let hash = self.hash(path, |b| ui.file_progress(b as u64)).unwrap();

        ui.end_file();

        hash
    }
}

fn hash_blake2b512<C: FnMut(usize)>(
    path: &Path,
    mut callback: C,
) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut blake2b512 = Blake2b512::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        blake2b512.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = blake2b512.finalize();

    Ok(hex::encode(hash))
}

fn hash_blake2s256<C: FnMut(usize)>(
    path: &Path,
    mut callback: C,
) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut blake2s256 = Blake2s256::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        blake2s256.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = blake2s256.finalize();

    Ok(hex::encode(hash))
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

fn hash_md5<C: FnMut(usize)>(path: &Path, mut callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut md5 = Md5::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        md5.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = md5.finalize();

    Ok(hex::encode(hash))
}

fn hash_sha1<C: FnMut(usize)>(path: &Path, mut callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha1 = Sha1::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        sha1.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = sha1.finalize();

    Ok(hex::encode(hash))
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

fn hash_sha3_256<C: FnMut(usize)>(path: &Path, mut callback: C) -> Result<String, util::FileError> {
    let mut file = util::open_file(path)?;
    let mut sha3_256 = Sha3_256::new();

    let mut buf = [0u8; BUFFER_SIZE];

    while let Ok(bytes) = file.read(&mut buf) {
        if bytes == 0 {
            break;
        }

        sha3_256.update(&buf[..bytes]);

        callback(bytes);
    }

    let hash = sha3_256.finalize();

    Ok(hex::encode(hash))
}
