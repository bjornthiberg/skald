use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde_json;

use super::JournalEntry;

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new(base_path: impl AsRef<Path>) -> io::Result<Self> {
        let base_path: PathBuf = base_path.as_ref().to_owned();
        fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    pub fn save_entry(&self, entry: &JournalEntry) -> io::Result<()> {
        let filename: String = format!("{}.json", entry.title());
        let path: PathBuf = self.base_path.join(filename);
        let content: String = serde_json::to_string_pretty(entry)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn list_entries(&self) -> io::Result<Vec<String>> {
        let entries: Vec<String> = fs::read_dir(&self.base_path)?
            .filter_map(|entry| entry.ok()) // filter out errors
            .filter_map(|entry| {
                entry
                    .path()
                    .file_stem() // Get filename without extension
                    .and_then(|name| name.to_str())
                    .map(String::from)
            })
            .collect();

        Ok(entries)
    }

    pub fn load_entry(&self, title: &str) -> io::Result<JournalEntry> {
        let path: PathBuf = self.base_path.join(format!("{}.json", title));
        let content: String = fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e: serde_json::Error| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn delete_entry(&self, title: &str) -> io::Result<()> {
        let path: PathBuf = self.base_path.join(format!("{}.json", title));
        fs::remove_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn storage_new_dir_is_created() -> io::Result<()> {
        let temp_dir: TempDir = TempDir::new()?;
        let storage: Storage = Storage::new(temp_dir.path())?;

        assert!(storage.base_path.exists());
        assert_eq!(storage.base_path, temp_dir.path());

        Ok(())
    }

    #[test]
    fn storage_entry_is_saved_to_file() -> io::Result<()> {
        let temp_dir: TempDir = tempfile::tempdir()?;
        let storage: Storage = Storage::new(temp_dir.path())?;

        let entry: JournalEntry = JournalEntry::new(
            "A Tale of Two Cities",
            "It was the best of times, it was the blurst of times.",
        );
        storage.save_entry(&entry)?;

        let expected_file_path: PathBuf = temp_dir.path().join("A Tale of Two Cities.json");
        let actual_content: String = fs::read_to_string(expected_file_path)?;

        assert_eq!(actual_content, serde_json::to_string_pretty(&entry)?);

        Ok(())
    }

    #[test]
    fn storage_lists_saved_entries() -> io::Result<()> {
        let temp_dir: TempDir = tempfile::tempdir()?;
        let storage: Storage = Storage::new(temp_dir.path())?;

        let entries: Vec<JournalEntry> = vec![
            JournalEntry::new("Entry 1", "Content 1"),
            JournalEntry::new("Entry 2", "Content 2"),
        ];

        for entry in &entries {
            storage.save_entry(entry)?;
        }

        let listed_entries: Vec<String> = storage.list_entries()?;

        assert_eq!(listed_entries, vec!["Entry 1", "Entry 2"]);

        Ok(())
    }

    #[test]
    fn storage_deletes_entry() -> io::Result<()> {
        let temp_dir: TempDir = tempfile::tempdir()?;
        let storage: Storage = Storage::new(temp_dir.path())?;

        let entry: JournalEntry = JournalEntry::new("Entry 1", "Content 1");
        storage.save_entry(&entry)?;

        // entry exists before deletion
        assert!(storage.list_entries()?.contains(&"Entry 1".to_string()));

        storage.delete_entry("Entry 1")?;

        // entry is deleted
        assert!(!storage.list_entries()?.contains(&"Entry 1".to_string()));

        Ok(())
    }
}
