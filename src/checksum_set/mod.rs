mod builder;
mod diff;
mod hasher;

use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

pub use self::builder::*;
pub use self::diff::*;

#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HashType {
    Crc32,
    Sha256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChecksumSet {
    pub hash_type: HashType,
    pub files: BTreeMap<String, String>,
}
