use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenBlockRequest {
    /// ID of the block to open
    pub block_id: Uuid,
}
