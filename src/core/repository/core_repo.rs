use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::core::error::CoreError;
use crate::core::index::Index;
use crate::core::index::index_entry::IndexEntry;
use crate::domain::model::types::Types;
use crate::domain::model::types::{FromDraft, HasId};
use crate::domain::path::vaults::Vaults;

/// Persistence layer for a single vault.
///
/// Owns the vault [`Index`] and is responsible for:
/// - type checks (entity type vs requested type)
/// - reading/writing/deleting JSON files
/// - keeping `index.json` consistent
pub struct CoreRepo {
    vaults: Vaults,
    index: Index,
}

impl CoreRepo {
    /// Opens a vault repository, ensuring required directories and `index.json` exist.
    pub fn open(vaults: Vaults) -> Result<Self, CoreError> {
        for dir in vaults.all_dir_paths() {
            fs::create_dir_all(&dir)?;
        }

        let index_path = vaults.index_path();
        if !index_path.exists() {
            fs::write(&index_path, "{}\n")?;
        }

        let index = Index::open(index_path)?;
        Ok(Self { vaults, index })
    }

    pub fn get_by_uuid<T>(&self, uuid: Uuid) -> Result<T, CoreError>
    where
        T: HasId + DeserializeOwned,
    {
        let entry = self
            .index
            .get_entity_by_uuid(&uuid)
            .ok_or(CoreError::NotFound)?;

        let expected = T::entity_type();
        if entry.entity_type != expected {
            return Err(CoreError::TypeMismatch {
                expected,
                found: entry.entity_type.clone(),
            });
        }

        let on_disk_path = self.resolve_on_disk_path(&entry.path)?;
        let content = fs::read_to_string(on_disk_path)?;
        let entity: T = serde_json::from_str(&content)?;
        Ok(entity)
    }

    /// Creates an entity file at `relative_path` (relative to vault base) and registers it in the index.
    pub fn create<T>(&mut self, relative_path: PathBuf, draft: T::Draft) -> Result<T, CoreError>
    where
        T: HasId + FromDraft + Serialize,
    {
        validate_relative_path(&relative_path)?;

        let uuid = Uuid::new_v4();
        let entity = T::from_draft(draft, uuid);

        let on_disk_path = self.vaults.base_path().join(&relative_path);
        if let Some(parent) = on_disk_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&entity)?;
        fs::write(on_disk_path, content)?;

        self.index.add_index(
            uuid,
            IndexEntry {
                path: relative_path,
                entity_type: T::entity_type(),
            },
        );
        self.index.save()?;

        Ok(entity)
    }

    /// Creates an entity using a default relative path derived from its type and uuid:
    /// - `notes/<uuid>.json`
    /// - `tests/<uuid>.json`
    /// - `dictionary/<uuid>.json`
    ///
    /// Callers only provide draft data; the repository returns the created entity (with id).
    pub fn create_auto<T>(&mut self, draft: T::Draft) -> Result<T, CoreError>
    where
        T: HasId + FromDraft + Serialize,
    {
        let uuid = Uuid::new_v4();
        let relative_path = default_relative_path_for_type(T::entity_type(), uuid);
        validate_relative_path(&relative_path)?;
        let entity = T::from_draft(draft, uuid);

        let on_disk_path = self.vaults.base_path().join(&relative_path);
        if let Some(parent) = on_disk_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&entity)?;
        fs::write(on_disk_path, content)?;

        self.index.add_index(
            uuid,
            IndexEntry {
                path: relative_path,
                entity_type: T::entity_type(),
            },
        );
        self.index.save()?;

        Ok(entity)
    }

    /// Updates an existing entity file using its uuid and the path stored in the index.
    pub fn update<T>(&mut self, entity: T) -> Result<(), CoreError>
    where
        T: HasId + Serialize,
    {
        let uuid = entity.id();
        let entry = self
            .index
            .get_entity_by_uuid(&uuid)
            .ok_or(CoreError::NotFound)?;

        let expected = T::entity_type();
        if entry.entity_type != expected {
            return Err(CoreError::TypeMismatch {
                expected,
                found: entry.entity_type.clone(),
            });
        }

        let on_disk_path = self.resolve_on_disk_path(&entry.path)?;
        let content = serde_json::to_string_pretty(&entity)?;
        fs::write(on_disk_path, content)?;

        self.index.save()?;
        Ok(())
    }

    /// Removes an entity file and its index entry by uuid.
    pub fn remove_by_uuid<T>(&mut self, uuid: Uuid) -> Result<(), CoreError>
    where
        T: HasId,
    {
        let entry = self
            .index
            .get_entity_by_uuid(&uuid)
            .ok_or(CoreError::NotFound)?;

        let expected = T::entity_type();
        if entry.entity_type != expected {
            return Err(CoreError::TypeMismatch {
                expected,
                found: entry.entity_type.clone(),
            });
        }

        let on_disk_path = self.resolve_on_disk_path(&entry.path)?;
        fs::remove_file(on_disk_path)?;

        self.index.remove_index_by_uuid(&uuid);
        self.index.save()?;
        Ok(())
    }

    /// Removes an entity by its relative path (relative to vault base).
    pub fn remove_by_path<T>(&mut self, relative_path: PathBuf) -> Result<(), CoreError>
    where
        T: HasId,
    {
        validate_relative_path(&relative_path)?;
        let uuid = self
            .index
            .get_uuid_by_path(&relative_path)
            .cloned()
            .ok_or(CoreError::NotFound)?;
        self.remove_by_uuid::<T>(uuid)
    }

    fn resolve_on_disk_path(&self, stored_path: &PathBuf) -> Result<PathBuf, CoreError> {
        if stored_path.is_absolute() {
            return Ok(stored_path.clone());
        }
        validate_relative_path(stored_path)?;
        Ok(self.vaults.base_path().join(stored_path))
    }
}

fn validate_relative_path(path: &Path) -> Result<(), CoreError> {
    if path.is_absolute() {
        return Err(CoreError::InvalidData("path must be relative".to_string()));
    }
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::ParentDir => {
                return Err(CoreError::InvalidData(
                    "path must not contain prefix/root/..".to_string(),
                ));
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    Ok(())
}

fn default_relative_path_for_type(entity_type: Types, uuid: Uuid) -> PathBuf {
    let dir = match entity_type {
        Types::Note => "notes",
        Types::Test => "tests",
        Types::Dictionary => "dictionary",
    };
    PathBuf::from(dir).join(format!("{uuid}.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::model::dictionary::{DictionaryDraft, Metadata};
    use crate::domain::model::note::{NoteDraft, NoteMetadata};
    use crate::domain::model::test::{Problem, TestDraft};

    fn temp_vault_base() -> PathBuf {
        std::env::temp_dir().join(format!("forgenotes-test-{}", Uuid::new_v4()))
    }

    #[test]
    fn create_note_writes_file_and_index() {
        let base = temp_vault_base();
        let vaults = Vaults::new(&base);
        let mut repo = CoreRepo::open(vaults).unwrap();

        let draft = NoteDraft {
            content: "hello".to_string(),
            title: "t".to_string(),
            description: "d".to_string(),
            metadata: NoteMetadata {
                tags: vec![],
                links: vec![],
                tests: vec![],
                dictionary: vec![],
                attachments: vec![],
                created_at: chrono::Utc::now(),
            },
        };

        let entity = repo
            .create::<crate::domain::model::note::Note>(PathBuf::from("notes/n1.json"), draft)
            .unwrap();

        let index_json = fs::read_to_string(base.join("index.json")).unwrap();
        assert!(index_json.contains(&entity.id.to_string()));
        assert!(base.join("notes/n1.json").exists());
    }

    #[test]
    fn get_type_mismatch() {
        let base = temp_vault_base();
        let vaults = Vaults::new(&base);
        let mut repo = CoreRepo::open(vaults).unwrap();

        let draft = DictionaryDraft {
            source_language: "en".to_string(),
            translate_language: "jp".to_string(),
            source_text: "a".to_string(),
            meanings: vec!["b".to_string()],
            translates: vec!["c".to_string()],
            use_cases: vec![],
            metadata: Metadata {
                tags: vec![],
                notes: vec![],
                tests: vec![],
                dictionary: vec![],
                attachments: vec![],
            },
        };

        let dict = repo
            .create::<crate::domain::model::dictionary::Dictionary>(
                PathBuf::from("dictionary/d1.json"),
                draft,
            )
            .unwrap();

        let err = repo
            .get_by_uuid::<crate::domain::model::note::Note>(dict.id)
            .unwrap_err();

        match err {
            CoreError::TypeMismatch { .. } => {}
            other => panic!("expected TypeMismatch, got {other:?}"),
        }
    }

    #[test]
    fn remove_by_uuid_removes_both() {
        let base = temp_vault_base();
        let vaults = Vaults::new(&base);
        let mut repo = CoreRepo::open(vaults).unwrap();

        let draft = TestDraft {
            title: "t".to_string(),
            note_uuids: vec![],
            dictionary_uuids: vec![],
            created_at: "now".to_string(),
            problems: vec![Problem::ExactAnswer {
                question: "q".to_string(),
                answer: "a".to_string(),
                hint: None,
            }],
        };

        let test = repo
            .create::<crate::domain::model::test::Test>(PathBuf::from("tests/t1.json"), draft)
            .unwrap();
        assert!(base.join("tests/t1.json").exists());

        repo.remove_by_uuid::<crate::domain::model::test::Test>(test.id)
            .unwrap();
        assert!(!base.join("tests/t1.json").exists());

        let index_json = fs::read_to_string(base.join("index.json")).unwrap();
        assert!(!index_json.contains(&test.id.to_string()));
    }

    #[test]
    fn reject_absolute_path() {
        let base = temp_vault_base();
        let vaults = Vaults::new(&base);
        let mut repo = CoreRepo::open(vaults).unwrap();

        let draft = NoteDraft {
            content: "hello".to_string(),
            title: "t".to_string(),
            description: "d".to_string(),
            metadata: NoteMetadata {
                tags: vec![],
                links: vec![],
                tests: vec![],
                dictionary: vec![],
                attachments: vec![],
                created_at: chrono::Utc::now(),
            },
        };

        let abs_path = base.join("notes/n.json");
        let err = repo
            .create::<crate::domain::model::note::Note>(abs_path, draft)
            .unwrap_err();

        match err {
            CoreError::InvalidData(_) => {}
            other => panic!("expected InvalidData, got {other:?}"),
        }
    }
}
