//! note file DTO (main kind of file)
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Note file main json structure
    pub id: Uuid,
    pub content: String, // note markdown plain text
    pub title: String,
    pub description: String,
    pub metadata: NoteMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMetadata {
    /// metadata for note
    pub tags: Vec<String>,
    pub links: Vec<Uuid>,
    pub tests: Vec<Uuid>,
    pub dictionary: Vec<Uuid>,
    pub attachemnts: Vec<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
