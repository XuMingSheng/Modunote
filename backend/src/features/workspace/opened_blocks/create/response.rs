use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenBlockResponse {
    pub block_id: Uuid,
    pub opened_at: DateTime<Utc>,
}
