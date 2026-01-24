use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::repositories::block_repository::CreateBlockDto;

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockRequest {
    pub title: String,
    pub content: String,
}

impl From<CreateBlockRequest> for CreateBlockDto {
    fn from(request: CreateBlockRequest) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            title: request.title,
            content: request.content,
        }
    }
}
