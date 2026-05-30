pub mod make;
pub mod note;

use std::path::Path;

pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, vault_base: &Path, args: &[String]) -> Result<(), String>;
}
