use serde;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockSearchRequest {
    pub query: String,
}
