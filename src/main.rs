use std::{convert::TryFrom, fmt::Display, path::PathBuf, process};

use cfg::Config;
use chrono::{Date, Utc};
use dialoguer::Editor;
use structopt::StructOpt;
use walkdir::WalkDir;

use crate::debug::dbg_info;

mod cfg;
mod cmd;
mod debug;
mod entry;

fn main() {
    let cfg: Config = Config::from_args();

    if cfg.debug() {
        println!("{}", dbg_info());
        process::exit(0);
    }

    match cfg.command() {
        cmd::Command::Edit { date } => edit(&cfg, date).unwrap(),
        cmd::Command::Search { terms } => search(&cfg, terms).unwrap(),
        cmd::Command::Log { entries } => log(&cfg, entries).unwrap(),
    };
}

fn edit(cfg: &Config, date: Option<Date<Utc>>) -> Result<(), std::io::Error> {
    let file: PathBuf = cfg.file(date);

    let content = match std::fs::read_to_string(&file) {
        Ok(content) => content,
        Err(_) => String::from(""),
    };

    let edit: Option<String> = Editor::new().extension(".md").trim_newlines(true).edit(&content)?;

    if let Some(content) = edit {
        create_parent(&file)?;
        std::fs::write(&file, content)?
    }

    Ok(())
}

fn create_parent(path: &PathBuf) -> std::io::Result<()> {
    let mut parent = path.clone();
    parent.pop();
    match parent.exists() {
        true => Ok(()),
        false => std::fs::create_dir_all(parent),
    }
}

fn log(cfg: &Config, entries: usize) -> std::io::Result<()> {
    let files: Vec<walkdir::DirEntry> = WalkDir::new(cfg.dir())
        .follow_links(false)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|p| p.file_type().is_file())
        .collect();

    files
        .into_iter()
        .rev()
        .take(entries)
        .filter_map(entry_to_string)
        .for_each(|line| println!("{}", line));

    Ok(())
}

fn entry_to_string(entry: walkdir::DirEntry) -> Option<String> {
    let name: String = entry.file_name().to_os_string().into_string().ok()?;
    std::fs::read_to_string(entry.path())
        .ok()
        .map(|content| format!("---{}---\n{}\n", name, content))
}

fn search(cfg: &Config, terms: Vec<String>) -> std::io::Result<()> {
    WalkDir::new(cfg.dir())
        .follow_links(false)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|d| d.ok())
        .filter_map(|f| Entry::try_from(f).ok())
        .filter(|e: &Entry| e.contains_any(&terms))
        .for_each(|e| println!("{}", e));

    Ok(())
}

struct Entry {
    pub name: String,
    pub content: String,
}

impl Entry {
    pub fn contains(&self, term: &str) -> bool {
        self.content.to_lowercase().contains(&term.to_lowercase())
    }

    pub fn contains_any(&self, terms: &Vec<String>) -> bool {
        terms.iter().any(|term| self.contains(&term))
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("---{}---\n{}\n", self.name, self.content))
    }
}

impl TryFrom<walkdir::DirEntry> for Entry {
    type Error = EntryError;

    fn try_from(value: walkdir::DirEntry) -> Result<Self, Self::Error> {
        let f = value.file_type();
        if f.is_dir() {
            return Err(EntryError::IsDir);
        } else if f.is_symlink() {
            return Err(EntryError::IsSymlink);
        }

        let name: String = match value.file_name().to_os_string().into_string() {
            Ok(name) => name,
            Err(_) => {
                return Err(EntryError::Other(String::from("Unable to convert name of file")))
            }
        };
        let content: String = match std::fs::read_to_string(value.path()) {
            Ok(content) => content,
            Err(e) => return Err(EntryError::Other(e.to_string())),
        };
        let entry = Entry { name, content };
        Ok(entry)
    }
}

enum EntryError {
    IsDir,
    IsSymlink,
    Other(String),
}
