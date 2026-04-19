use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Types {
    Note,
    Test,
    Dictionary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// tags for searching and categorization
    pub tags: Vec<String>,
    /// related notes
    pub related_notes: Vec<Uuid>,
    /// related tests
    pub related_tests: Vec<Uuid>,
    /// related dictionaries
    pub related_dictionaries: Vec<Uuid>,
    /// related attachments
    pub related_attachments: Vec<Uuid>,
    /// creation timestamp
    pub created_at: DateTime<Utc>,
    /// last update timestamp
    pub updated_at: DateTime<Utc>,
    /// version for future migrations
    pub version: u32,
}

pub trait HasId {
    fn id(&self) -> Uuid;
    fn entity_type() -> Types;
    fn metadata(&self) -> &Metadata;
    fn metadata_mut(&mut self) -> &mut Metadata;
}

/// Build an entity from a draft that does not contain an id.
pub trait FromDraft: Sized {
    type Draft;
    fn from_draft(draft: Self::Draft, id: Uuid) -> Self;
}
