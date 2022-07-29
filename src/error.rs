use std::borrow::Cow;
use std::io;

use crate::util;

#[derive(Debug)]
pub enum CliErrorKind {
    Other,
}

impl CliErrorKind {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Other => 101,
        }
    }
}

#[derive(Debug)]
pub struct CliError {
    pub kind: CliErrorKind,
    pub description: Cow<'static, str>,
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError {
            kind: CliErrorKind::Other,
            description: Cow::Owned(error.to_string()),
        }
    }
}

impl From<util::FileError> for CliError {
    fn from(error: util::FileError) -> Self {
        CliError {
            kind: CliErrorKind::Other,
            description: Cow::Owned(format!("File not found: {}", error.path.display())),
        }
    }
}
