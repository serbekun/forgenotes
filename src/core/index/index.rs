use crate::core::model::types::Types;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, path::PathBuf};

use uuid::Uuid;

pub struct Index {
    index: HashMap<Uuid, IndexEntry>,
    path_to_index_file: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: PathBuf,
    pub entity_type: Types,
}

impl Index {
    pub fn new(path_to_index_file: PathBuf) -> Self {
        match read_index_hash_map_from_file(&path_to_index_file) {
            Ok(index_map) => {
                return Self {
                    index: index_map,
                    path_to_index_file,
                };
            }
            Err(e) => {
                eprintln!("Failed to load index: {}", e);
                Self {
                    index: HashMap::new(),
                    path_to_index_file,
                }
            }
        }
    }

    /// Saves the index hashmap to a file on disk.
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
    /// ```
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
    /// let index = Index::new(path_to_index_file.clone());
    /// index.save().unwrap();
    /// assert!(std::fs::metadata(path_to_index_file).is_ok());
    /// ```
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.index)?;
        fs::write(&self.path_to_index_file, content)?;
        Ok(())
    }

    ///
    /// Add uuid to index registry
    ///
    /// #
    ///
    ///
    pub fn add_uuid(&mut self, uuid: Uuid, index_entry: IndexEntry) {
        self.index.insert(uuid, index_entry);
    }

    pub fn remove_uuid(&mut self, uuid: Uuid) {
        self.index.remove(&uuid);
    }

    pub fn get_by_uuid(&self, uuid: Uuid) -> Option<&IndexEntry> {
        self.index.get(&uuid)
    }
}

fn read_index_hash_map_from_file(
    path: &PathBuf,
) -> Result<HashMap<Uuid, IndexEntry>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let map: HashMap<Uuid, IndexEntry> = serde_json::from_str(&content)?;

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::vaults::Vaults;

    #[test]
    fn save() {
        // building path to index file
        let base_path = "test_vaults";
        let vaults = Vaults::new(base_path);
        let path_to_index_file = PathBuf::from(vaults.index_path());

        // call save method
        let index = Index::new(path_to_index_file.clone());
        index.save().unwrap();

        assert!(std::fs::metadata(path_to_index_file).is_ok());
    }
}
