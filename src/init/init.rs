use crate::init::init_interface::InitInterface;
use crate::init::infrastructure::fs::init_infrastructure_fs::InitInfrastructureFs;
use std::env;

pub fn init() -> Result<(), String> {
    let base_path = env::current_dir()
        .map_err(|err| format!("failed to resolve current directory: {err}"))?;

    let init_infrastructure_fs = InitInfrastructureFs::new(base_path);
    init_infrastructure_fs.init()
}
