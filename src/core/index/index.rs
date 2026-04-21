use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use bimap::BiHashMap;
use uuid::Uuid;

use crate::core::error::CoreError;
use crate::core::index::index_entry::IndexEntry;

pub struct Index {
    index: BiHashMap<Uuid, IndexEntry>,
    path_to_index_file: PathBuf,
}

impl Index {
    /// Opens an index from disk (or returns an empty index if file doesn't exist).
    pub fn open(path_to_index_file: PathBuf) -> Result<Self, CoreError> {
        let index_map = Self::read_index_hash_map_from_file(&path_to_index_file)?;
        Ok(Self {
            index: index_map,
            path_to_index_file,
        })
    }

    /// Saves the index to a file on disk.
    ///
    /// Serializes the in-memory index into JSON and writes it to the given path.
    ///
    /// # Arguments
    /// * `path` - Path to the file where the index will be saved.
    ///
    /// # Errors
    /// Returns an error if serialization fails or if the file cannot be written.
    /// See [`std::error::Error`].
    ///
    /// # Examples
    /// ```ignore
    /// use std::path::PathBuf;
    /// use crate::path::vaults::Vaults;
    /// use std::fs;
    ///
    /// // building path to index file
    /// let base_path = "test_vaults";
    /// let vaults = Vaults::new(base_path);
    /// let path_to_index_file = PathBuf::from(vaults.index_path());
    ///
    /// // call save method
    /// let index = Index::open(path_to_index_file.clone()).unwrap();
    /// index.save().unwrap();
    /// assert!(std::fs::metadata(path_to_index_file).is_ok());
    /// ```ignore
    pub fn save(&self) -> Result<(), CoreError> {
        let map: HashMap<Uuid, IndexEntry> = self
            .index
            .iter()
            .map(|(uuid, entry)| (uuid.clone(), entry.clone()))
            .collect();
        let content = serde_json::to_string_pretty(&map)?;

        if let Some(parent) = self.path_to_index_file.parent() {
            fs::create_dir_all(parent)?;
            let tmp_path = parent.join(format!(
                ".{}.tmp-{}",
                self.path_to_index_file
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("index.json"),
                Uuid::new_v4()
            ));
            fs::write(&tmp_path, content)?;
            if self.path_to_index_file.exists() {
                fs::remove_file(&self.path_to_index_file)?;
            }
            fs::rename(tmp_path, &self.path_to_index_file)?;
        } else {
            fs::write(&self.path_to_index_file, content)?;
        }
        Ok(())
    }

    ///
    /// Add uuid to index registry.
    ///
    /// # Arguments
    /// * `uuid` entity object uuid.
    /// * `index_entry` entity metadata for index.
    ///
    /// # Example
    /// ```ignore
    /// let uuid: Uuid = Uuid::new_v4();
    ///
    /// // making data that will be included to index
    /// let index_entry: IndexEntry = IndexEntry {
    ///     path_buf: PathBuf::from("relative/path/to/entity.json"),
    ///     entity: Entity::Note,
    /// };
    ///
    /// // building path to index file
    /// let base_path = "test_vaults";
    /// let vaults = Vaults::new(base_path);
    /// let path_to_index_file = PathBuf::from(vaults.index_path());
    ///
    /// // call save method
    /// let index = Index::new(path_to_index_file.clone());
    /// index.add_uuid(uuid, index_entry);
    ///
    /// assert!(index_entry, index.get_by_uuid(uuid));
    ///
    /// ```ignore
    pub fn add_index(&mut self, uuid: Uuid, index_entry: IndexEntry) {
        let _ = self.index.insert(uuid, index_entry);
    }

    ///
    /// Remove index by uuid from index.
    ///
    /// # Arguments
    /// * `uuid` uuid of object that will be deleted.
    ///
    pub fn remove_index_by_uuid(&mut self, uuid: &Uuid) {
        let _ = self.index.remove_by_left(uuid);
    }

    /// Remove index by path from index.
    ///
    /// # Arguments
    /// * `path` PathBuf object, path to relative object.
    ///
    pub fn remove_index_by_path(&mut self, path: &PathBuf) {
        let uuid_to_remove = self
            .index
            .iter()
            .find(|(_, entry)| &entry.path == path)
            .map(|(uuid, _)| uuid.clone());
        if let Some(uuid) = uuid_to_remove {
            let _ = self.index.remove_by_left(&uuid);
        }
    }

    ///
    /// Return index entity by uuid.
    ///
    /// # Arguments
    /// * `uuid` uuid of object that will be uuid.
    ///
    ///
    pub fn get_entity_by_uuid(&self, uuid: &Uuid) -> Option<&IndexEntry> {
        self.index.get_by_left(uuid)
    }

    ///
    /// Returns index entity uuid by relative path to entity
    ///
    /// # Arguments
    /// * `path`: PathBuf relative path to entity
    pub fn get_uuid_by_path(&self, path: &PathBuf) -> Option<&Uuid> {
        self.index
            .iter()
            .find(|(_, entry)| &entry.path == path)
            .map(|(uuid, _)| uuid)
    }

    ///
    /// Help function for load index hashmap from file.
    ///
    /// # Arguments
    /// * `path` PathBuf path to index.json where saved index hashmap.
    ///
    fn read_index_hash_map_from_file(
        path: &PathBuf,
    ) -> Result<BiHashMap<Uuid, IndexEntry>, CoreError> {
        if !path.exists() {
            return Ok(BiHashMap::new());
        }
        let content = fs::read_to_string(path)?;
        let map: HashMap<Uuid, IndexEntry> =
            serde_json::from_str(&content).map_err(|_| CoreError::IndexCorrupted)?;
        let mut bimap = BiHashMap::new();
        for (k, v) in map {
            let _ = bimap.insert(k, v);
        }
        Ok(bimap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::path::vaults::Vaults;

    #[test]
    fn save() {
        // building path to index file
        let base_path = "test_vaults";
        let vaults = Vaults::new(base_path);
        let path_to_index_file = PathBuf::from(vaults.index_path());

        // call save method
        let index = Index::open(path_to_index_file.clone()).unwrap();
        index.save().unwrap();

        assert!(std::fs::metadata(path_to_index_file).is_ok());
    }
}
