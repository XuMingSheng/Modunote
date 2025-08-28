use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateChildLinkRequest {
    /// ID of the child block to link from this block
    pub child_block_id: Uuid,
}
