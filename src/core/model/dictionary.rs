use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationEntry {
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
pub struct Metadata {
    pub tags: Vec<Uuid>,
    pub notes: Vec<Uuid>,
    pub tests: Vec<Uuid>,
    pub dictionary: Vec<Uuid>,
    pub attachments: Vec<Uuid>,
}
