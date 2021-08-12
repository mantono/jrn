use std::{path::PathBuf, process};

use cfg::Config;
use chrono::{Date, Utc};
use clap::Clap;
use dialoguer::Editor;

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
        cmd::Command::History { entries } => todo!(),
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
