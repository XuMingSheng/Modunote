use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateParentLinkRequest {
    /// ID of the parent block to link to this block
    pub parent_block_id: Uuid,
}
