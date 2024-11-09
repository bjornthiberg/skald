use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JournalEntry {
    title: String,
    content: String,
    created_at: DateTime<Utc>,
}

impl JournalEntry {
    pub fn new(title: &str, content: &str) -> Self {
        let now = Utc::now();
        Self {
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.created_at
    }

}
/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_journal_entry_when_valid_title_and_content() {
        let entry = JournalEntry::new("Title", "Content");
        assert_eq!(entry.title(), "Title");
        assert_eq!(entry.content(), "Content");
    }

    #[test]
    fn write_to_file_creates_file() -> std::io::Result<()> {
        use tempdir::TempDir;

        let dir = TempDir::new("test_dir")?;
        let file_path = dir.path().join("write.txt");
        let entry = JournalEntry::new("Title", "Content");

        entry.write_to_file(&file_path)?;

        assert!(file_path.exists());

        let file_content = std::fs::read_to_string(&file_path)?;
        let expected_content = "Title\nContent";

        assert_eq!(file_content, expected_content);

        Ok(())
    }

    #[test]
    fn load_from_file_valid_entry() -> std::io::Result<()> {
        use tempdir::TempDir;
        let dir = TempDir::new("test_dir")?;
        let file_path = dir.path().join("write.txt");

        let original_entry = JournalEntry::new("Loaded Title", "Loaded Content");
        original_entry.write_to_file(&file_path)?;

        let loaded_entry = JournalEntry::load_from_file(&file_path)?;

        assert_eq!(loaded_entry.title(), original_entry.title());
        assert_eq!(loaded_entry.content(), original_entry.content());

        Ok(())
    }

    #[test]
    fn load_from_empty_file_should_fail() -> std::io::Result<()> {
        use tempdir::TempDir;

        let dir = TempDir::new("test_dir")?;
        let file_path = dir.path().join("empty.txt");

        File::create(&file_path)?;

        let result = JournalEntry::load_from_file(&file_path);

        assert!(result.is_err());

        Ok(())
    }
}
 */
