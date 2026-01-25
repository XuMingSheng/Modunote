use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Block {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Block {
    pub fn new(title: &str, content: &str) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
