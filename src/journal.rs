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
