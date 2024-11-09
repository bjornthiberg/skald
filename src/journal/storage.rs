use std::path::{Path, PathBuf};
use std::io;
use std::fs::{self, write};
use serde_json;

use super::JournalEntry;

pub struct Storage {
    base_path: PathBuf,
}


impl Storage {
    pub fn new(base_path: impl AsRef<Path>) -> io::Result<Self> {
        let base_path: PathBuf = base_path.as_ref().to_owned();
        fs::create_dir_all(&base_path)?;
        Ok(Self {base_path})
    }

    pub fn save_entry(&self, entry: &JournalEntry) -> io::Result<()> {
        let filename: String = format!("{}.json", entry.title());
        let path: PathBuf = self.base_path.join(filename);
        let content: String = serde_json::to_string_pretty(entry)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn list_entries() {}

    pub fn load_entry() {}

    pub fn delete_entry() {}
}
