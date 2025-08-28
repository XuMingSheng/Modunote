use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelatedLinkResponse {
    pub link_id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
    pub created_at: DateTime<Utc>,
}
