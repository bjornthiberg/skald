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

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
