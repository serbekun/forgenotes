use crate::core::index::index_ref::IndexRef;
use crate::core::repository::repository_errors::RepositoryError;
use std::path::PathBuf;
use uuid::Uuid;

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
    /// Add note file to registry
    ///
    /// # Arguments
    /// * `path` relative path to note.
    /// * `entity` note object that will be saved.
    ///
    fn add_file(&mut self, path: PathBuf, entity: Self::Entity);
}
