use std::{convert::TryFrom, fmt::Display, path::PathBuf, process};

use cfg::Config;
use chrono::{Date, Utc};
use crossterm::style::Stylize;
use dialoguer::Editor;
use structopt::StructOpt;
use termimad::MadSkin;
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
        cmd::Command::Search { terms, limit } => search(&cfg, terms, limit).unwrap(),
        cmd::Command::Log { limit } => log(&cfg, limit).unwrap(),
    };
}

/// Return the number of bytes that were written.
fn edit(cfg: &Config, date: Option<Date<Utc>>) -> Result<usize, std::io::Error> {
    let file: PathBuf = cfg.file(date);

    let content: String =
        if file.exists() { std::fs::read_to_string(&file)? } else { String::from("") };

    let edit: Option<String> = Editor::new().extension(".md").trim_newlines(true).edit(&content)?;

    match edit {
        Some(content) => {
            create_parent(&file)?;
            let bytes: usize = content.bytes().len();
            std::fs::write(&file, content)?;
            Ok(bytes)
        }
        None => Ok(0),
    }
}

fn create_parent(path: &PathBuf) -> std::io::Result<()> {
    let mut parent = path.clone();
    parent.pop();
    match parent.exists() {
        true => Ok(()),
        false => std::fs::create_dir_all(parent),
    }
}

/// Returns the number of entries printed, if successful, else a std::io::Error
fn log(cfg: &Config, limit: usize) -> std::io::Result<usize> {
    let entries: Vec<Entry> = WalkDir::new(cfg.dir())
        .follow_links(false)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|p| p.file_type().is_file())
        .filter_map(|f| Entry::try_from(f).ok())
        .collect();

    let entries: Vec<Entry> = entries.into_iter().rev().take(limit).collect();

    Ok(print_entries(entries))
}

/// Returns the number of entries found and printed, if successful, else a std::io::Error
fn search(cfg: &Config, terms: Vec<String>, limit: usize) -> std::io::Result<usize> {
    let entries: Vec<Entry> = WalkDir::new(cfg.dir())
        .follow_links(false)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|d| d.ok())
        .filter_map(|f| Entry::try_from(f).ok())
        .filter(|e: &Entry| e.contains_any(&terms))
        .take(limit)
        .collect();

    Ok(print_entries(entries))
}

fn print_entries(entries: Vec<Entry>) -> usize {
    let mut skin = MadSkin::default();
    skin.strikeout.add_attr(crossterm::style::Attribute::CrossedOut);
    skin.strikeout.set_fg(crossterm::style::Color::Grey);
    skin.set_headers_fg(crossterm::style::Color::Cyan);

    let length: usize = entries.len();
    for e in entries {
        print_entry(e, &skin)
    }

    length
}

fn print_entry(entry: Entry, skin: &MadSkin) {
    let name: String = entry.name;
    println!("\n{} {} {}", "┈┈".dark_grey(), name.yellow(), "┈┈┈┈┈".dark_grey());
    skin.print_text(&entry.content);
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
