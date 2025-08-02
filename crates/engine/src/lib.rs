use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub content: String,        // markdown with custom syntax
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct View {
    pub id: Uuid,
    pub name: String,
    pub layout: String,          // for simplicity, a JSON string (future: serde_json::Value)
    pub block_ids: Vec<Uuid>,    // ordered block references
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub data: Vec<u8>,          // store in db or fs, see storage
    pub block_id: Option<Uuid>, // block association (if any)
}
