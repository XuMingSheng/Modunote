use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BlockCreate {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct BlockUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
}
