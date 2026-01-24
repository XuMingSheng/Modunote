use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::BlockRelatedLink;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockRelatedLinkResponse {
    pub id: Uuid,
    pub block_ids: [Uuid; 2],
    pub created_at: DateTime<Utc>,
}

impl From<BlockRelatedLink> for CreateBlockRelatedLinkResponse {
    fn from(link: BlockRelatedLink) -> Self {
        Self {
            id: link.id,
            block_ids: [link.block_a_id, link.block_b_id],
            created_at: link.created_at,
        }
    }
}
