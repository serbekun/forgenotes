use std::path::PathBuf;

use crate::cli::interpreter::Interpreter;
use crate::cli::tokenizer;

pub fn run_command(command: String) {
    let tokens = tokenizer::tokenize_string(&command);
    let vault_base: PathBuf = PathBuf::from("vaults");
    let interpreter = Interpreter::new();
    if let Err(e) = interpreter.execute(&vault_base, &tokens) {
        eprintln!("error: {e}");
    }
}
