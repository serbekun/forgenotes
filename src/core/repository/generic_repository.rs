use std::path::PathBuf;
use std::fs;

use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::core::repository::repository_errors::RepositoryError;
use crate::core::repository::repository::Repository;
use crate::core::index::index_entry::IndexEntry;
use crate::core::index::index_ref::IndexRef;
use crate::core::model::types::*;

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
                .get_entity_by_uuid(&uuid)
                .ok_or(RepositoryError::NotFound)?;

            if entry.entity_type != self.entity_type {
                return Err(RepositoryError::TypeMismatch {
                    expected: self.entity_type.clone(),
                    found: entry.entity_type.clone(),
                });
            }
            entry.clone()
        };

        let content = fs::read_to_string(&index_entry.path).map_err(RepositoryError::IoError)?;

        let entity: T = serde_json::from_str(&content).map_err(RepositoryError::Deserialize)?;

        Ok(entity)
    }

    fn create(
        &mut self,
        path: PathBuf,
        draft: <Self::Entity as FromDraft>::Draft,
    ) -> Result<Self::Entity, RepositoryError>
    where
        Self::Entity: FromDraft + Serialize,
    {
        let uuid = Uuid::new_v4();
        let entity = T::from_draft(draft, uuid);

        let content = serde_json::to_string_pretty(&entity).map_err(RepositoryError::Serialize)?;
        fs::write(&path, content).map_err(RepositoryError::IoError)?;

        let index_entry = IndexEntry {
            path,
            entity_type: self.entity_type.clone(),
        };
        self.index.add_index(uuid, index_entry);
        self.index
            .save()
            .map_err(|err| RepositoryError::IndexSave(err.to_string()))?;

        Ok(entity)
    }

    fn update(&mut self, entity: Self::Entity) -> Result<(), RepositoryError>
    where
        Self::Entity: HasId + Serialize,
    {
        let uuid = entity.id();
        let index_entry = self
            .index
            .get_entity_by_uuid(uuid)
            .ok_or(RepositoryError::NotFound)?;

        if index_entry.entity_type != self.entity_type {
            return Err(RepositoryError::TypeMismatch {
                expected: self.entity_type.clone(),
                found: index_entry.entity_type.clone(),
            });
        }

        let content = serde_json::to_string_pretty(&entity).map_err(RepositoryError::Serialize)?;
        fs::write(&index_entry.path, content).map_err(RepositoryError::IoError)?;
        self.index
            .save()
            .map_err(|err| RepositoryError::IndexSave(err.to_string()))?;

        Ok(())
    }

    fn remove_file_by_uuid(&mut self, uuid: Uuid) -> Result<(), RepositoryError> {
        let entity = self
            .index
            .get_entity_by_uuid(uuid)
            .ok_or(RepositoryError::NotFound)?;

        if entity.entity_type != self.entity_type {
            return Err(RepositoryError::TypeMismatch {
                expected: self.entity_type.clone(),
                found: entity.entity_type.clone(),
            });
        }

        let path = entity.path.clone();

        fs::remove_file(&path).map_err(RepositoryError::IoError)?;

        self.index.remove_index_by_uuid(uuid);
        self.index
            .save()
            .map_err(|err| RepositoryError::IndexSave(err.to_string()))?;

        Ok(())
    }

    fn remove_file_by_path(&mut self, path: &PathBuf) -> Result<(), RepositoryError> {
        let uuid = self.index.get_uuid_by_path(path).ok_or(RepositoryError::NotFound)?;
        self.remove_file_by_uuid(uuid)?;
        Ok(())
    }

    

}
