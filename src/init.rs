use std::env;
use std::fs;
use std::fs::create_dir_all;

use crate::domain::path::vaults::Vaults;

pub fn init() -> Result<(), String> {
    let base_path =
        env::current_dir().map_err(|err| format!("failed to resolve current directory: {err}"))?;

    let vaults = Vaults::new(base_path);
    for path in vaults.all_dir_paths() {
        create_dir_all(&path)
            .map_err(|err| format!("failed to create directory {}: {err}", path.display()))?;
    }

    let index_path = vaults.index_path();
    if !index_path.exists() {
        fs::write(&index_path, "{}\n").map_err(|err| {
            format!(
                "failed to create index file {}: {err}",
                index_path.display()
            )
        })?;
    }

    Ok(())
}
