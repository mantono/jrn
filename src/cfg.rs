use std::path::PathBuf;

use crate::cmd::Command;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jrn", author, about)]
pub struct Config {
    /// Print debug information
    ///
    /// Print debug information about current build for binary, useful for when an issue is
    /// encountered and reported
    #[structopt(short = "D", long)]
    debug: bool,

    /// Namespace
    ///
    /// Namespace to use. For exmaple "work", "private" or similar.
    #[structopt(short, long, default_value = "default")]
    namespace: String,

    /// Command
    ///
    /// Command to execute
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

impl Config {
    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn command(&self) -> Command {
        self.cmd.clone().unwrap_or_default()
    }

    fn data_dir(&self) -> PathBuf {
        let options: Vec<Option<PathBuf>> = vec![
            dirs_next::data_local_dir(),
            dirs_next::data_dir(),
            dirs_next::document_dir(),
            dirs_next::home_dir(),
        ];

        let mut root: PathBuf =
            options.into_iter().flatten().next().expect("Unable to find a data directory");

        root.push("jrn");
        root
    }

    pub fn dir(&self) -> PathBuf {
        let mut dir = self.data_dir();
        dir.push(&self.namespace);
        if !dir.exists() {
            std::fs::create_dir_all(&dir).unwrap();
        }
        dir
    }

    pub fn file(&self, file_name: Option<String>) -> PathBuf {
        let file_name = file_name.unwrap_or_else(crate::cmd::gen_id);
        let mut dir: PathBuf = self.dir();
        dir.push(format!("{file_name}.md"));
        dir
    }
}
