use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use storage::{BlockOpenRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      delete,
      path = "/api/workspace/opened-blocks/{block_id}",
      tag = "workspace",
      params(
          ("block_id" = uuid::Uuid, Path, description = "Block ID to close")
      ),
      responses(
          (status = 204, description = "Block closed successfully"),
          (status = 404, description = "Block is not opened"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn close_block(
    State(repos): State<RepositoryProvider>,
    Path(block_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let is_opened = repos.block_opens.is_opened(block_id).await.map_err(|e| {
        error!("Failed to check if block {} is opened: {e}", block_id);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !is_opened {
        return Err(StatusCode::NOT_FOUND);
    }

    repos.block_opens.close(block_id).await.map_err(|e| {
        error!("Failed to close block {}: {e}", block_id);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}
