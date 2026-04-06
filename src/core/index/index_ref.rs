use super::index::Index;
use crate::core::index::index_entry::IndexEntry;
use std::cell::RefCell;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Default)]
pub struct IndexRef {
    inner: RefCell<Index>,
}

impl IndexRef {
    pub fn new(path_to_index_file: PathBuf) -> Self {
        Self {
            inner: RefCell::new(Index::new(path_to_index_file)),
        }
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, Index> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, Index> {
        self.inner.borrow_mut()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.borrow().save()
    }

    pub fn add_uuid(&self, uuid: Uuid, index_entry: IndexEntry) {
        self.inner.borrow_mut().add_uuid(uuid, index_entry);
    }

    pub fn remove_uuid(&self, uuid: Uuid) {
        self.inner.borrow_mut().remove_uuid(uuid);
    }

    pub fn get_by_uuid(&self, uuid: Uuid) -> Option<IndexEntry> {
        self.inner.borrow().get_by_uuid(uuid).cloned()
    }
}
