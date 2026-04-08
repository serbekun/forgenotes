use crate::core::model::types::Types;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    IoError(std::io::Error),
    Deserialize(serde_json::Error),
    Serialize(serde_json::Error),
    IndexSave(String),
    TypeMismatch { expected: Types, found: Types },
}
