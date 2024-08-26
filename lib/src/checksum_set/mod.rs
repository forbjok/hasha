mod builder;
mod diff;
mod hasher;
mod load;
mod verify;
mod write;

use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

pub use self::builder::*;
pub use self::diff::*;

#[derive(Copy, Clone, Debug, Deserialize, EnumString, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HashType {
    Blake2b512,
    Blake2s256,
    Blake3,
    Crc32,
    Md5,
    Sha1,
    Sha256,
    Sha3_256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileInfo {
    pub size: u64,
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChecksumSet {
    pub hash_type: HashType,
    pub files: BTreeMap<String, FileInfo>,
}
