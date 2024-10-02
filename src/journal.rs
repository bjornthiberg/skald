use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct JournalEntry {
    title: String,
    content: String,
}

impl JournalEntry {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file_path: P) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;

        writeln!(file, "{}", self.title)?;

        file.write_all(self.content.as_bytes())?;

        Ok(())
    }
}

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
}
