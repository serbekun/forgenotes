use std::path::Path;

use uuid::Uuid;

use crate::cli::commands::Command;
use crate::core::service::files::FilesService;
use crate::domain::model::note::{Note, NoteDraft, NoteMetadata};
use crate::domain::path::vaults::Vaults;

pub struct NoteCommand;

impl Command for NoteCommand {
    fn name(&self) -> &'static str {
        "/note"
    }

    fn execute(&self, vault_base: &Path, args: &[String]) -> Result<(), String> {
        run(vault_base, args)
    }
}

fn run(vault_base: &Path, args: &[String]) -> Result<(), String> {
    if args.is_empty() || args.iter().any(|t| t == "--help" || t == "-h") {
        print_usage();
        return Ok(());
    }

    let mut service =
        FilesService::open(Vaults::new(vault_base)).map_err(|e| format!("open vault: {e}"))?;

    match args[0].as_str() {
        "create" => {
            let draft = parse_create(&args[1..])?;
            let uuid = service
                .create_note(draft)
                .map_err(|e| format!("create note: {e}"))?;
            println!("{uuid}");
            Ok(())
        }
        "list" => {
            parse_list(args, &service);

            Ok(())
        }

        "get" => {
            let uuid = parse_uuid_positional(&args[1..])?;
            let note = service.get_note(uuid).map_err(|e| format!("get note: {e}"))?;
            print_note(&note);
            Ok(())
        }

        "update" => {
            let (uuid, patch) = parse_update(&args[1..])?;
            let mut note = service.get_note(uuid).map_err(|e| format!("get note: {e}"))?;
            apply_patch(&mut note, patch);
            service
                .update_note(note)
                .map_err(|e| format!("update note: {e}"))?;
            Ok(())
        }
        "delete" | "del" | "rm" => {
            let uuid = parse_uuid_positional(&args[1..])?;
            service
                .remove_note(uuid)
                .map_err(|e| format!("delete note: {e}"))?;
            Ok(())
        }
        other => Err(format!(
            "unknown subcommand `{other}`. Try: /note --help"
        )),
    }


}

fn print_usage() {
    println!(
        r#"notes CRUD :
  /note create --title <text> [--description <text>] [--content <text>] [--tags <a,b,c>]
  /note get <uuid>
  /note update <uuid> [--title <text>] [--description <text>] [--content <text>] [--tags <a,b,c>]
  /note delete <uuid>
"#
    );
}

fn parse_create(args: &[String]) -> Result<NoteDraft, String> {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut content: Option<String> = None;
    let mut tags: Option<Vec<String>> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                title = Some(next_value(args, &mut i, "--title")?);
            }
            "--description" => {
                description = Some(next_value(args, &mut i, "--description")?);
            }
            "--content" => {
                content = Some(next_value(args, &mut i, "--content")?);
            }
            "--tags" => {
                let raw = next_value(args, &mut i, "--tags")?;
                tags = Some(parse_tags(&raw));
            }
            other => return Err(format!("unknown argument `{other}`. Try: /note create --help")),
        }
        i += 1;
    }

    let title = title.ok_or("missing required flag `--title`".to_string())?;
    let description = description.unwrap_or_default();
    let content = content.unwrap_or_default();
    let tags = tags.unwrap_or_default();

    Ok(NoteDraft {
        content,
        title,
        description,
        metadata: NoteMetadata {
            tags,
            links: vec![],
            tests: vec![],
            dictionary: vec![],
            attachments: vec![],
            created_at: chrono::Utc::now(),
        },
    })
}

#[derive(Default)]
struct NotePatch {
    title: Option<String>,
    description: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
}

fn parse_list(args: &[String], service: &FilesService) {
    let mut tags: Vec<String> = vec![];

    if args.is_empty() {
        let notes = service.all_notes();
        for note in notes {
            println!("{}: {}", note.title, note.id);
        }
        return;
    }

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--tags" => {
                let raw = next_value(args, &mut i, "--tags");
                match raw {
                    Ok(s) => tags = parse_tags(&s),
                    Err(e) => eprintln!("{e}"),
                }
            }
            other => println!("unknown argument '{other}'"),
        }
        i += 1;
    }

    if tags.is_empty() {
        let notes = service.all_notes();
        for note in notes {
            println!("{}: {}", note.title, note.id);
        }
    } else {
        let notes = service.all_notes();
        for note in notes {
            if note.metadata.tags.iter().any(|t| tags.contains(t)) {
                println!("{}: {}", note.title, note.id);
            }
        }
    }
}

fn parse_update(args: &[String]) -> Result<(Uuid, NotePatch), String> {
    if args.is_empty() {
        return Err("usage: /note update <uuid> [--title ...] [--description ...] [--content ...] [--tags ...]".to_string());
    }

    let uuid = Uuid::parse_str(&args[0]).map_err(|_| "invalid uuid".to_string())?;
    let mut patch = NotePatch::default();

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                patch.title = Some(next_value(args, &mut i, "--title")?);
            }
            "--description" => {
                patch.description = Some(next_value(args, &mut i, "--description")?);
            }
            "--content" => {
                patch.content = Some(next_value(args, &mut i, "--content")?);
            }
            "--tags" => {
                let raw = next_value(args, &mut i, "--tags")?;
                patch.tags = Some(parse_tags(&raw));
            }
            other => return Err(format!("unknown argument `{other}`. Try: /note update --help")),
        }
        i += 1;
    }

    if patch.title.is_none()
        && patch.description.is_none()
        && patch.content.is_none()
        && patch.tags.is_none()
    {
        return Err("nothing to update (no flags provided)".to_string());
    }

    Ok((uuid, patch))
}

fn apply_patch(note: &mut Note, patch: NotePatch) {
    if let Some(v) = patch.title {
        note.title = v;
    }
    if let Some(v) = patch.description {
        note.description = v;
    }
    if let Some(v) = patch.content {
        note.content = v;
    }
    if let Some(v) = patch.tags {
        note.metadata.tags = v;
    }
}

fn parse_uuid_positional(args: &[String]) -> Result<Uuid, String> {
    if args.len() != 1 {
        return Err("expected exactly one <uuid>".to_string());
    }
    Uuid::parse_str(&args[0]).map_err(|_| "invalid uuid".to_string())
}

fn next_value(args: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    let value_idx = *i + 1;
    if value_idx >= args.len() {
        return Err(format!("missing value for `{flag}`"));
    }
    *i = value_idx;
    Ok(args[value_idx].clone())
}

fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn print_note(note: &Note) {
    println!("id: {}", note.id);
    println!("title: {}", note.title);
    println!("description: {}", note.description);
    println!("tags: {}", note.metadata.tags.join(", "));
    println!("content:\n{}", note.content);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    fn temp_vault_base() -> PathBuf {
        std::env::temp_dir().join(format!("forgenotes-cli-test-{}", Uuid::new_v4()))
    }

    #[test]
    fn create_get_update_delete_roundtrip() {
        let base = temp_vault_base();

        let create_args = vec![
            "create".to_string(),
            "--title".to_string(),
            "t".to_string(),
            "--description".to_string(),
            "d".to_string(),
            "--content".to_string(),
            "c".to_string(),
            "--tags".to_string(),
            "a,b".to_string(),
        ];
        run(&base, &create_args).unwrap();
        let service = FilesService::open(Vaults::new(&base)).unwrap();

        // We only printed uuid; fetch it from index.json.
        let index_json = std::fs::read_to_string(base.join("index.json")).unwrap();
        let value: serde_json::Value = serde_json::from_str(&index_json).unwrap();
        let uuid_str = value
            .as_object()
            .and_then(|obj| obj.keys().next())
            .ok_or("index.json empty")
            .unwrap();
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let note = service.get_note(uuid).unwrap();
        assert_eq!(note.title, "t");
        assert_eq!(note.description, "d");
        assert_eq!(note.content, "c");
        assert_eq!(note.metadata.tags, vec!["a".to_string(), "b".to_string()]);

        let update_args = vec![
            "update".to_string(),
            uuid.to_string(),
            "--title".to_string(),
            "t2".to_string(),
            "--tags".to_string(),
            "x".to_string(),
        ];
        run(&base, &update_args).unwrap();

        let note2 = service.get_note(uuid).unwrap();
        assert_eq!(note2.title, "t2");
        assert_eq!(note2.metadata.tags, vec!["x".to_string()]);

        let delete_args = vec!["delete".to_string(), uuid.to_string()];
        run(&base, &delete_args).unwrap();

        let service = FilesService::open(Vaults::new(&base)).unwrap();
        let err = service.get_note(uuid).unwrap_err();
        assert!(matches!(err, crate::core::error::CoreError::NotFound));
    }
}
