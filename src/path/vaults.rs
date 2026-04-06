//! Module for get fs path locations
//! for notes, tests, dictionary
use std::path::PathBuf;

pub struct Vaults {
    base_path: String,
}

impl Vaults {
    /// create new Vaults object.
    ///
    /// # Arguments
    /// * `base_path` base folder path.
    ///
    /// # Warning
    /// * `base_path` give string without `/` in end.
    pub fn new(base_path: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Return vaults base path
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    /// Return formatted path to notes with included base path
    pub fn notes_path(&self) -> String {
        format!("{}/notes", self.base_path())
    }

    /// Return formatted path to tests with included base path
    pub fn tests_path(&self) -> String {
        format!("{}/tests", self.base_path())
    }

    /// Return formatted path to tests with included base path
    pub fn dictionary_path(&self) -> String {
        format!("{}/dictionary", self.base_path())
    }

    /// Return formatted path to index file with included base path
    pub fn index_path(&self) -> String {
        format!("{}/index.json", self.base_path())
    }

    ///
    /// Return all dirs needed inside base folder.
    /// 
    /// # Return
    /// Vec<PathBuf> with all folders paths.
    /// 
    pub fn all_dir_paths(&self) -> Vec<PathBuf> {
        vec![
            self.notes_path().into(),
            self.tests_path().into(),
            self.dictionary_path().into(),
            self.index_path().into(),
        ]
    }
}
