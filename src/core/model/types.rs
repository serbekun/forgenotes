use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Types {
    Note,
    Test,
    Dictionary,
}

pub trait HasId {
    fn id(&self) -> Uuid;
    fn entity_type() -> Types;
}

/// Build an entity from a draft that does not contain an id.
pub trait FromDraft: Sized {
    type Draft;
    fn from_draft(draft: Self::Draft, id: Uuid) -> Self;
}
