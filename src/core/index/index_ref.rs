use std::cell::RefCell;
use std::path::PathBuf;

use uuid::Uuid;

use crate::core::index::index_entry::IndexEntry;
use super::index::Index;

#[derive(Default)]
/// Shared, interior-mutable wrapper around [`Index`].
///
/// Provides a `RefCell`-backed API for borrowing and mutating the index
/// without requiring `&mut self` on the wrapper itself.
pub struct IndexRef {
    inner: RefCell<Index>,
}

impl IndexRef {
    /// Creates a new [`IndexRef`] backed by an [`Index`] loaded from disk.
    ///
    /// # Arguments
    /// * `path_to_index_file` - Path to the index file on disk.
    pub fn new(path_to_index_file: PathBuf) -> Self {
        Self {
            inner: RefCell::new(Index::new(path_to_index_file)),
        }
    }

    /// Borrows the underlying [`Index`] immutably.
    pub fn borrow(&self) -> std::cell::Ref<'_, Index> {
        self.inner.borrow()
    }

    /// Borrows the underlying [`Index`] mutably.
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, Index> {
        self.inner.borrow_mut()
    }

    /// Saves the index to disk.
    ///
    /// See [`Index::save`] for details.
    ///
    /// # Errors
    /// Returns an error if serialization fails or if the file cannot be written.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.borrow().save()
    }

    /// Adds a uuid and its index entry to the registry.
    ///
    /// # Arguments
    /// * `uuid` - Entity uuid.
    /// * `index_entry` - Entity metadata stored in the index.
    pub fn add_index(&self, uuid: Uuid, index_entry: IndexEntry) {
        self.inner.borrow_mut().add_index(uuid, index_entry);
    }

    /// Removes an index entry by uuid.
    ///
    /// # Arguments
    /// * `uuid` - Entity uuid to remove.
    pub fn remove_index_by_uuid(&self, uuid: Uuid) {
        self.inner.borrow_mut().remove_index_by_uuid(&uuid);
    }

    /// Removes an index entry by path.
    ///
    /// # Arguments
    /// * `path` - Relative path to the entity.
    pub fn remove_index_by_path(&self, path: &PathBuf) {
        self.inner.borrow_mut().remove_index_by_path(path);
    }

    /// Returns an entity from the index by uuid.
    ///
    /// # Arguments
    /// * `uuid` - Entity uuid to fetch.
    pub fn get_entity_by_uuid(&self, uuid: Uuid) -> Option<IndexEntry> {
        self.inner.borrow().get_entity_by_uuid(&uuid).cloned()
    }

    /// Returns an entity uuid by its relative path.
    ///
    /// # Arguments
    /// * `path` - Relative path to the entity.
    pub fn get_uuid_by_path(&self, path: &PathBuf) -> Option<Uuid> {
        self.inner.borrow().get_uuid_by_path(path).cloned()
    }

}
