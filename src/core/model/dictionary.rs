use crate::core::model::types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    pub id: Uuid,
    pub source_language: String,
    pub translate_language: String,
    pub source_text: String,

    pub meanings: Vec<String>,
    pub translates: Vec<String>,
    pub use_cases: Vec<String>,

    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryDraft {
    pub source_language: String,
    pub translate_language: String,
    pub source_text: String,

    pub meanings: Vec<String>,
    pub translates: Vec<String>,
    pub use_cases: Vec<String>,

    pub metadata: Metadata,
}

impl HasId for Dictionary {
    fn id(&self) -> Uuid {
        self.id
    }
    fn entity_type() -> Types {
        Types::Dictionary
    }
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

impl FromDraft for Dictionary {
    type Draft = DictionaryDraft;

    fn from_draft(draft: Self::Draft, id: Uuid) -> Self {
        Self {
            id,
            source_language: draft.source_language,
            translate_language: draft.translate_language,
            source_text: draft.source_text,
            meanings: draft.meanings,
            translates: draft.translates,
            use_cases: draft.use_cases,
            metadata: draft.metadata,
        }
    }
}
