use std::{convert::TryFrom, fmt::Display};

#[derive(Debug)]
pub struct Entry {
    name: String,
    content: String,
}

impl Entry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

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

pub enum EntryError {
    IsDir,
    IsSymlink,
    Other(String),
}
