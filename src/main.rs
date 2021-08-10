use std::process;

use cfg::Config;
use clap::Clap;

use crate::debug::dbg_info;

mod cfg;
mod cmd;
mod debug;
mod entry;

fn main() {
    let cfg: Config = Config::parse();

    println!("{:?}", cfg.command());

    if cfg.debug() {
        println!("{}", dbg_info());
        process::exit(0);
    }
}
