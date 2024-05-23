use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    /// Edit journal
    ///
    /// Edit or create a journal entry
    Edit {
        /// Entry to edit
        ///
        /// Entry to edit. If no name is given, a new journal entry will be created
        #[structopt()]
        entry: Option<String>,
    },

    /// Search entries
    ///
    /// Search journal entries
    Search {
        #[structopt()]
        terms: Vec<String>,
        #[structopt(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show history
    ///
    /// Show last journal entries
    Log {
        #[structopt(short, long, default_value = "5")]
        limit: usize,
    },
}

impl Default for Command {
    fn default() -> Self {
        Command::Edit {
            entry: Some(gen_id()),
        }
    }
}

pub fn gen_id() -> String {
    let mut gen = flakeid::gen::FlakeGen::with_mac_addr().expect("Unable to create generator");
    gen.next().map(|id| format!("{id:x}")).expect("Failed to generate a flake id")
}
