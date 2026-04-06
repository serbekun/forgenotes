use crate::init::init_interface::InitInterface;
use crate::path::vaults::Vaults;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct InitInfrastructureFs {
    base_path: PathBuf,
}

impl InitInfrastructureFs {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}

impl InitInterface for InitInfrastructureFs {
    fn init(&self) -> Result<(), String> {
        let base_path = self.base_path.to_string_lossy().to_string();
        let vaults = Vaults::new(base_path);

        for path in vaults.all_dir_paths() {
            create_dir_all(&path)
                .map_err(|err| format!("failed to create directory {}: {err}", path.display()))?;
        }

        Ok(())
    }
}
