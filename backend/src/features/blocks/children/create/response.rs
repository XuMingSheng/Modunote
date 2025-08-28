use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateChildLinkResponse {
    pub link_id: Uuid,
    pub parent_block_id: Uuid,
    pub child_block_id: Uuid,
    pub created_at: DateTime<Utc>,
}
