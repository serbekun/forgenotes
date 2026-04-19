use std::path::PathBuf;

use serde::Serialize;
use uuid::Uuid;

use crate::core::error::CoreError;
use crate::core::index::IndexRef;
use crate::core::model::types::{FromDraft, HasId};

pub trait EntityRepository<'a> {
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
    /// [crate::core::error::CoreError]
    ///
    fn get_by_uuid(&self, uuid: Uuid) -> Result<Self::Entity, CoreError>;

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
    ) -> Result<Self::Entity, CoreError>
    where
        Self::Entity: FromDraft + Serialize;

    ///
    /// Update entity file on disk using its id and stored index path.
    ///
    /// # Arguments
    /// * `entity` object with existing id.
    ///
    fn update(&mut self, entity: Self::Entity) -> Result<(), CoreError>
    where
        Self::Entity: HasId + Serialize;

    ///
    /// Remove entity file from registry and disk.
    /// 
    /// # Arguments
    /// * `uuid` entity uuid.
    /// 
    fn remove_file_by_uuid(&mut self, uuid: Uuid) -> Result<(), CoreError>;

    ///
    /// Remove entity file from registry and disk.
    /// 
    /// # Arguments
    /// * `path` relative path to file.
    /// 
    fn remove_file_by_path(&mut self, path: &PathBuf) -> Result<(), CoreError>;
}
