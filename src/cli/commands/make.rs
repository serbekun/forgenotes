use std::path::Path;

use super::Command;

pub struct MakeCommand;

impl Command for MakeCommand {
    fn name(&self) -> &'static str {
        "/make"
    }

    fn execute(&self, _vault_base: &Path, args: &[String]) -> Result<(), String> {
        if args.is_empty() {
            println!("Usage: /make <object type>")
        }
        Ok(())
    }
}
