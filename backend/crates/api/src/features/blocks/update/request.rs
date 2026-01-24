use serde::Deserialize;
use utoipa::ToSchema;

use storage::repositories::block_repository::UpdateBlockDto;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlockRequest {
    /// Updated block title
    pub title: Option<String>,
    /// Updated block content in markdown
    pub content: Option<String>,
}

impl From<UpdateBlockRequest> for UpdateBlockDto {
    fn from(request: UpdateBlockRequest) -> Self {
        Self {
            title: request.title,
            content: request.content,
        }
    }
}
