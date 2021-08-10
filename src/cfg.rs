use crate::cmd::Command;

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "jrn", author, about)]
pub struct Config {
    /// Print debug information
    ///
    /// Print debug information about current build for binary, useful for when an issue is
    /// encountered and reported
    #[clap(short = 'D', long)]
    debug: bool,
    /// Command
    ///
    /// Command to execute
    #[clap(subcommand)]
    cmd: Option<Command>,
}

impl Config {
    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn command(&self) -> Command {
        self.cmd.clone().unwrap_or_else(|| Command::default())
    }
}
