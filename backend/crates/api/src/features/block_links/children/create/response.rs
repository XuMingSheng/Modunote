use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::BlockDirectionalLink;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockChildLinkResponse {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<BlockDirectionalLink> for CreateBlockChildLinkResponse {
    fn from(link: BlockDirectionalLink) -> Self {
        Self {
            id: link.id,
            block_from_id: link.block_from_id,
            block_to_id: link.block_to_id,
            created_at: link.created_at,
        }
    }
}
