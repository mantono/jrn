use std::{ffi::OsString, fmt::format, fs::DirEntry, path::PathBuf, process};

use cfg::Config;
use chrono::{Date, Utc};
use clap::Clap;
use dialoguer::Editor;
use walkdir::WalkDir;

use crate::debug::dbg_info;

mod cfg;
mod cmd;
mod debug;
mod entry;

fn main() {
    let cfg: Config = Config::parse();

    if cfg.debug() {
        println!("{}", dbg_info());
        process::exit(0);
    }

    match cfg.command() {
        cmd::Command::Edit { date } => edit(&cfg, date).unwrap(),
        cmd::Command::Search { terms } => todo!(),
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
