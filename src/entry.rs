use std::str::FromStr;

use chrono::Utc;

#[derive(Debug)]
struct Entry {
    date: chrono::Date<Utc>,
    content: String,
}

impl Entry {
    pub fn new() -> Self {
        Entry {
            date: Utc::today(),
            content: String::from(""),
        }
    }

    pub fn from(date: chrono::Date<Utc>, content: String) -> Self {
        Entry { date, content }
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entry = Entry {
            date: Utc::today(),
            content: s.to_string(),
        };
        Ok(entry)
    }
}
