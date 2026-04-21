use std::path::PathBuf;

use uuid::Uuid;

use crate::core::error::CoreError;
use crate::core::repository::CoreRepo;
use crate::domain::model::dictionary::{Dictionary, DictionaryDraft};
use crate::domain::model::note::{Note, NoteDraft};
use crate::domain::model::test::{Test, TestDraft};
use crate::domain::path::vaults::Vaults;

/// Facade over the core persistence layer.
///
/// Other services should depend on this API instead of talking to repositories/index directly.
pub struct FilesService {
    repo: CoreRepo,
}

impl FilesService {
    pub fn open(vaults: Vaults) -> Result<Self, CoreError> {
        Ok(Self {
            repo: CoreRepo::open(vaults)?,
        })
    }

    // Notes
    pub fn create_note(&mut self, draft: NoteDraft) -> Result<Uuid, CoreError> {
        let note = self.repo.create_auto::<Note>(draft)?;
        Ok(note.id)
    }

    pub fn get_note(&self, uuid: Uuid) -> Result<Note, CoreError> {
        self.repo.get_by_uuid::<Note>(uuid)
    }

    pub fn update_note(&mut self, note: Note) -> Result<(), CoreError> {
        self.repo.update::<Note>(note)
    }

    pub fn remove_note(&mut self, uuid: Uuid) -> Result<(), CoreError> {
        self.repo.remove_by_uuid::<Note>(uuid)
    }

    // Tests
    pub fn create_test(&mut self, draft: TestDraft) -> Result<Uuid, CoreError> {
        let test = self.repo.create_auto::<Test>(draft)?;
        Ok(test.id)
    }

    pub fn get_test(&self, uuid: Uuid) -> Result<Test, CoreError> {
        self.repo.get_by_uuid::<Test>(uuid)
    }

    pub fn update_test(&mut self, test: Test) -> Result<(), CoreError> {
        self.repo.update::<Test>(test)
    }

    pub fn remove_test(&mut self, uuid: Uuid) -> Result<(), CoreError> {
        self.repo.remove_by_uuid::<Test>(uuid)
    }

    // Dictionary
    pub fn create_dictionary(&mut self, draft: DictionaryDraft) -> Result<Uuid, CoreError> {
        let dictionary = self.repo.create_auto::<Dictionary>(draft)?;
        Ok(dictionary.id)
    }

    pub fn get_dictionary(&self, uuid: Uuid) -> Result<Dictionary, CoreError> {
        self.repo.get_by_uuid::<Dictionary>(uuid)
    }

    pub fn update_dictionary(&mut self, dictionary: Dictionary) -> Result<(), CoreError> {
        self.repo.update::<Dictionary>(dictionary)
    }

    pub fn remove_dictionary(&mut self, uuid: Uuid) -> Result<(), CoreError> {
        self.repo.remove_by_uuid::<Dictionary>(uuid)
    }

    // Escape hatch (advanced)
    pub fn create_at_path<T>(&mut self, relative_path: PathBuf, draft: T::Draft) -> Result<T, CoreError>
    where
        T: crate::domain::model::types::HasId + crate::domain::model::types::FromDraft + serde::Serialize,
    {
        self.repo.create::<T>(relative_path, draft)
    }
}
