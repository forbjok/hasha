mod builder;
mod diff;
mod hasher;
mod verify;

use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

pub use self::builder::*;
pub use self::diff::*;
pub use self::verify::*;

#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HashType {
    Blake2b512,
    Blake2s256,
    Crc32,
    Md5,
    Sha1,
    Sha256,
    Sha3_256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChecksumSet {
    pub hash_type: HashType,
    pub files: BTreeMap<String, String>,
}
