use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use storage::{BlockRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      delete,
      path = "/api/blocks/{id}",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID")
      ),
      responses(
          (status = 204, description = "Block deleted successfully"),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn delete_block(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {id}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    repos.blocks.delete_by_id(id).await.map_err(|e| {
        error!("Failed to delete block {id}: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}
