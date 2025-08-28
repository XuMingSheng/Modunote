use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}
