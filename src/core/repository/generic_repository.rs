use crate::core::repository::repository_errors::RepositoryError;
use crate::core::repository::repository::Repository;
use crate::core::index::index_entry::IndexEntry;
use crate::core::index::index_ref::IndexRef;
use std::fs;

use crate::core::model::types::*;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use uuid::Uuid;

pub struct GenericRepository<'a, T: HasId + DeserializeOwned> {
    index: &'a IndexRef,
    entity_type: Types,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: HasId + DeserializeOwned> Repository<'a> for GenericRepository<'a, T> {
    type Entity = T;

    fn name() -> &'static str {
        std::any::type_name::<T>()
    }

    fn new(index: &'a IndexRef) -> Self {
        Self {
            index,
            entity_type: T::entity_type(),
            _phantom: std::marker::PhantomData,
        }
    }

    fn get_by_uuid(&self, uuid: Uuid) -> Result<T, RepositoryError> {
        let index_entry = {
            let guard = self.index.borrow();
            let entry = guard
                .get_by_uuid(&uuid)
                .ok_or(RepositoryError::NotFound)?;

            if entry.entity_type != self.entity_type {
                return Err(RepositoryError::TypeMismatch {
                    expected: self.entity_type.clone(),
                    found: entry.entity_type.clone(),
                });
            }
            entry.clone()
        };

        let content = fs::read_to_string(&index_entry.path).map_err(RepositoryError::Io)?;

        let entity: T = serde_json::from_str(&content).map_err(RepositoryError::Deserialize)?;

        Ok(entity)
    }

    fn add_file(&mut self, path: PathBuf, entity: T) {
        let uuid = entity.id();
        let index_entry = IndexEntry {
            path,
            entity_type: self.entity_type.clone(),
        };
        self.index.add_uuid(uuid, index_entry);
    }
}
