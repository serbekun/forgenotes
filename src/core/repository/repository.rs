use std::path::PathBuf;

use serde::Serialize;
use uuid::Uuid;

use crate::core::model::types::{FromDraft, HasId};
use crate::core::repository::repository_errors::RepositoryError;
use crate::core::index::index_ref::IndexRef;

pub trait Repository<'a> {
    type Entity;

    fn new(index: &'a IndexRef) -> Self;

    /// Return repository name
    fn name() -> &'static str;

    ///
    /// Return entity object by uuid
    /// entity object from module [crate::core::model::note]
    /// # Arguments
    /// * `uuid` object uuid.
    ///
    /// # Returns
    /// Note object with uuid that be given in parameters
    ///
    /// # Error
    /// [crate::core::repository::repository_errors]
    ///
    fn get_by_uuid(&self, uuid: Uuid) -> Result<Self::Entity, RepositoryError>;

    ///
    /// Create entity from draft (without id). Repository generates UUID.
    ///
    /// # Arguments
    /// * `path` relative path to entity file.
    /// * `draft` entity data without id.
    ///
    fn create(
        &mut self,
        path: PathBuf,
        draft: <Self::Entity as FromDraft>::Draft,
    ) -> Result<Self::Entity, RepositoryError>
    where
        Self::Entity: FromDraft + Serialize;

    ///
    /// Update entity file on disk using its id and stored index path.
    ///
    /// # Arguments
    /// * `entity` object with existing id.
    ///
    fn update(&mut self, entity: Self::Entity) -> Result<(), RepositoryError>
    where
        Self::Entity: HasId + Serialize;

    ///
    /// Remove entity file from registry and disk.
    /// 
    /// # Arguments
    /// * `uuid` entity uuid.
    /// 
    fn remove_file_by_uuid(&mut self, uuid: Uuid) -> Result<(), RepositoryError>;

    ///
    /// Remove entity file from registry and disk.
    /// 
    /// # Arguments
    /// * `path` relative path to file.
    /// 
    fn remove_file_by_path(&mut self, path: &PathBuf) -> Result<(), RepositoryError>;
}
