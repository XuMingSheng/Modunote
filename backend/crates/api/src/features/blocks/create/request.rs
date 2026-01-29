use chrono::Utc;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::Block;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockRequest {
    pub title: String,
    pub content: String,
}

impl From<CreateBlockRequest> for Block {
    fn from(request: CreateBlockRequest) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();

        Self {
            id,
            title: request.title,
            content: request.content,
            created_at: now,
            updated_at: now,
        }
    }
}
