use chrono::{Date, NaiveDate, ParseError, Utc};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    /// Edit journal
    ///
    /// Edit or create a journal entry
    Edit {
        /// Date to edit
        ///
        /// Date to edit. If no date is given, the journal entry that is opened will be for the
        /// current date.
        #[structopt(parse(try_from_str = parse_date))]
        date: Option<chrono::Date<Utc>>,
    },

    /// Search entries
    ///
    /// Search journal entries
    Search {
        #[structopt()]
        terms: Vec<String>,
    },

    /// Show history
    ///
    /// Show last journal entries
    Log {
        #[structopt(default_value = "5")]
        entries: usize,
    },

    /// Sync data
    ///
    /// Sync entries recorded with Git repository
    #[cfg(feature = "git2")]
    Sync,
}

impl Default for Command {
    fn default() -> Self {
        Command::Edit {
            date: Some(Utc::today()),
        }
    }
}

fn parse_date(input: &str) -> Result<chrono::Date<Utc>, String> {
    let date: Result<chrono::Date<Utc>, ParseError> =
        NaiveDate::parse_from_str(input, "%Y-%m-%d").map(|date| Date::<Utc>::from_utc(date, Utc));

    match date {
        Ok(date) => match date.cmp(&Utc::today()) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Ok(date),
            std::cmp::Ordering::Greater => Err(String::from("date is in the future")),
        },
        Err(e) => Err(e.to_string()),
    }
}
