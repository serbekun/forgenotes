use crate::core::model::types::Types;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct IndexEntry {
    pub path: PathBuf,
    pub entity_type: Types,
}
