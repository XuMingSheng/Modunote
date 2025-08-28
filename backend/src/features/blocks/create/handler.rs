use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::{request::CreateBlockRequest, response::CreateBlockResponse};
use storage::{BlockCreate, BlockRepositoryTrait, RepositoryProvider};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks",
    tag = "blocks",
    request_body = CreateBlockRequest,
    responses(
        (status = 201, description = "Block created successfully", body = CreateBlockResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn create_block(
    State(repos): State<RepositoryProvider>,
    Json(request): Json<CreateBlockRequest>,
) -> Result<(StatusCode, Json<CreateBlockResponse>), StatusCode> {
    let title = request.title.clone();

    let block_create = BlockCreate {
        id: Uuid::new_v4(),
        title: request.title,
        content: request.content,
    };

    let block = repos.blocks.create(block_create).await.map_err(|e| {
        error!("Failed to create block {title}: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = CreateBlockResponse {
        id: block.id,
        title: block.title,
        content: block.content,
        created_at: block.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
