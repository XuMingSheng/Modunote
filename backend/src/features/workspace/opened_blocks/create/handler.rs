use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::{request::OpenBlockRequest, response::OpenBlockResponse};
use storage::{BlockOpenRepositoryTrait, BlockRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      post,
      path = "/api/workspace/opened-blocks",
      tag = "workspace",
      request_body = OpenBlockRequest,
      responses(
          (status = 201, description = "Block opened successfully", body = OpenBlockResponse),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn open_block(
    State(repos): State<RepositoryProvider>,
    Json(request): Json<OpenBlockRequest>,
) -> Result<(StatusCode, Json<OpenBlockResponse>), StatusCode> {
    repos
        .blocks
        .get_by_id(request.block_id)
        .await
        .map_err(|e| {
            error!("Failed to get block {}: {e}", request.block_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let block_open = repos
        .block_opens
        .open(request.block_id)
        .await
        .map_err(|e| {
            error!("Failed to open block {}: {e}", request.block_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = OpenBlockResponse {
        block_id: block_open.block_id,
        opened_at: block_open.opened_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
