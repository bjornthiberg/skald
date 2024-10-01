use std::fs::File;
use std::io::Write;

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

    pub fn write_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all((self.title.clone() + &"\n".to_string() + &self.content).as_bytes())?;
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
}
