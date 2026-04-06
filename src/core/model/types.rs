use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Types {
    Note,
    Test,
    Dictionary,
}

pub trait HasId {
    fn id(&self) -> Uuid;
    fn entity_type() -> Types;
}