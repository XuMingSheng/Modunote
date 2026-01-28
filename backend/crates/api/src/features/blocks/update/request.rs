use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}
