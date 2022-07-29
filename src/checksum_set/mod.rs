mod builder;
mod diff;

use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

pub use self::builder::*;
pub use self::diff::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HashType {
    Sha256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChecksumSet {
    pub hash_type: HashType,
    pub files: BTreeMap<String, String>,
}
