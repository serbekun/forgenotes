use crate::core::model::types::Types;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    TypeMismatch { expected: Types, found: Types },
}
