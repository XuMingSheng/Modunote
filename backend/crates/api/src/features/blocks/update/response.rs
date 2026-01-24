use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::Block;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

impl From<Block> for UpdateBlockResponse {
    fn from(block: Block) -> Self {
        Self {
            id: block.id,
            title: block.title,
            content: block.content,
            updated_at: block.updated_at,
        }
    }
}
