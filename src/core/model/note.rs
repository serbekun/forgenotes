//! note file DTO (main kind of file)
use crate::core::model::types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Note file main json structure
    pub id: Uuid,
    pub content: String, // note markdown plain text
    pub title: String,
    pub description: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDraft {
    pub content: String,
    pub title: String,
    pub description: String,
    pub metadata: Metadata,
}

impl HasId for Note {
    fn id(&self) -> Uuid {
        self.id
    }
    fn entity_type() -> Types {
        Types::Note
    }
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}

impl FromDraft for Note {
    type Draft = NoteDraft;

    fn from_draft(draft: Self::Draft, id: Uuid) -> Self {
        Self {
            id,
            content: draft.content,
            title: draft.title,
            description: draft.description,
            metadata: draft.metadata,
        }
    }
}
