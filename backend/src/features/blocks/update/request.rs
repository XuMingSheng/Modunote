use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockRequest {
    /// Updated block title
    pub title: Option<String>,
    /// Updated block content in markdown
    pub content: Option<String>,
}
