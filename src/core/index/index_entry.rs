use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::model::types::Types;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct IndexEntry {
    pub path: PathBuf,
    pub entity_type: Types,
}
