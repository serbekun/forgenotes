//! Module for get fs path locations
//! for notes, tests, dictionary
use std::path::PathBuf;

pub struct Vaults {
    base_path: PathBuf,
}

impl Vaults {
    /// create new Vaults object.
    ///
    /// # Arguments
    /// * `base_path` base folder path.
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Return vaults base path
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Return formatted path to notes with included base path
    pub fn notes_path(&self) -> PathBuf {
        self.base_path.join("notes")
    }

    /// Return formatted path to tests with included base path
    pub fn tests_path(&self) -> PathBuf {
        self.base_path.join("tests")
    }

    /// Return formatted path to dictionary with included base path
    pub fn dictionary_path(&self) -> PathBuf {
        self.base_path.join("dictionary")
    }

    /// Return formatted path to index file with included base path
    pub fn index_path(&self) -> PathBuf {
        self.base_path.join("index.json")
    }

    ///
    /// Return all dirs needed inside base folder.
    ///
    /// # Return
    /// Vec<PathBuf> with all folders paths.
    ///
    pub fn all_dir_paths(&self) -> Vec<PathBuf> {
        vec![self.notes_path(), self.tests_path(), self.dictionary_path()]
    }
}
