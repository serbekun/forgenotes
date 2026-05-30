use std::collections::HashMap;
use std::path::Path;

use crate::cli::commands::note::NoteCommand;
use crate::cli::commands::make::MakeCommand;
use crate::cli::commands::Command;

struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "/help"
    }

    fn execute(&self, _vault_base: &Path, _args: &[String]) -> Result<(), String> {
        println!("Available command: /note, /make");
        Ok(())
    }
}

struct ClearCommand;

impl Command for ClearCommand {
    fn name(&self) -> &'static str {
        "/clear"
    }

    fn execute(&self, _vault_base: &Path, _args: &[String]) -> Result<(), String> {
        clearscreen::clear().expect("failed to clear screen");
        Ok(())
    }
}

pub struct Interpreter {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut this = Self {
            commands: HashMap::new(),
        };
        this.register(HelpCommand);
        this.commands.insert("help".to_string(), Box::new(HelpCommand));
        this.register(ClearCommand);
        this.commands.insert("/cls".to_string(), Box::new(ClearCommand));
        this.register(NoteCommand);
        this.register(MakeCommand);
        this
    }

    pub fn register<C: Command + 'static>(&mut self, command: C) {
        self.commands.insert(command.name().to_string(), Box::new(command));
    }

    pub fn execute(&self, vault_base: &Path, tokens: &[String]) -> Result<(), String> {
        if tokens.is_empty() {
            return Ok(());
        }

        match self.commands.get(tokens[0].as_str()) {
            Some(cmd) => cmd.execute(vault_base, &tokens[1..]),
            None => Err(format!("unknown command `{}`. Try: /help", tokens[0])),
        }
    }
}
