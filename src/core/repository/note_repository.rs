use crate::core::index::index::{Index, IndexEntry};
use crate::core::model::note::Note;
use crate::core::model::types::Types;
use crate::core::repository::repository::Repository;
use crate::core::repository::repository_errors::RepositoryError;
use uuid::Uuid;

pub struct NoteRepository<'a> {
    index: &'a mut Index,
}

impl<'a> Repository<'a> for NoteRepository<'a> {
    type Entity = Note;

    fn name() -> &'static str {
        "note_repository"
    }

    fn new(index: &'a mut Index) -> Self {
        Self { index }
    }

    fn get_by_uuid(&self, uuid: Uuid) -> Result<Self::Entity, RepositoryError> {
        let index_entry = self
            .index
            .get_by_uuid(uuid)
            .ok_or_else(|| RepositoryError::NotFound)?;

        if index_entry.entity_type != Types::Note {
            return Err(RepositoryError::TypeMismatch {
                expected: Types::Note,
                found: index_entry.entity_type.clone(),
            });
        }

        let content = std::fs::read_to_string(&index_entry.path).map_err(RepositoryError::Io)?;

        let note: Note = serde_json::from_str(&content).map_err(RepositoryError::Deserialize)?;

        Ok(note)
    }

    fn add_file(&mut self, path: std::path::PathBuf, entity: Self::Entity) {
        let uuid: Uuid = entity.id;

        let index_entry: IndexEntry = IndexEntry {
            path,
            entity_type: Types::Note,
        };

        self.index.add_uuid(uuid, index_entry);
    }
}
