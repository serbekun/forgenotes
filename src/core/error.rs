use crate::domain::model::types::Types;
use std::fmt;

#[derive(Debug)]
pub enum CoreError {
    NotFound,
    Io(std::io::Error),
    Serde(serde_json::Error),
    InvalidData(String),
    IndexCorrupted,
    IndexSave(String),
    TypeMismatch { expected: Types, found: Types },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::NotFound => write!(f, "Entity not found"),
            CoreError::Io(e) => write!(f, "IO error: {}", e),
            CoreError::Serde(e) => write!(f, "Serialization error: {}", e),
            CoreError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            CoreError::IndexCorrupted => write!(f, "Index is corrupted"),
            CoreError::IndexSave(msg) => write!(f, "Failed to save index: {}", msg),
            CoreError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Type mismatch: expected {:?}, found {:?}",
                    expected, found
                )
            }
        }
    }
}

impl std::error::Error for CoreError {}

impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}
