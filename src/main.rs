pub mod core;
pub mod domain;
pub mod init;
pub mod cli;

fn main() {
    if let Err(e) = init::init() {
        eprintln!("init error: {e}");
        std::process::exit(1);
    }

    let mut rl = rustyline::DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("failed to create editor: {e}");
        std::process::exit(1);
    });

    println!("For exit type '/q' or '/exit'");

    loop {
        let line = match rl.readline("> ") {
            Ok(l) => l,
            Err(rustyline::error::ReadlineError::Interrupted | rustyline::error::ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };
        rl.add_history_entry(&line).ok();
        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "/exit" || trimmed == "/q" {
            break;
        }
        cli::interface::run_command(trimmed);
    }
}
