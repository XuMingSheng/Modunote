use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::{AppState, features::error::HandlerError};
use storage::repositories::BlockRepository;
use storage::repositories::block_repository::BlockRepositoryError;

#[utoipa::path(
      delete,
      path = "/api/blocks/{id}",
      tag = "blocks",
      responses(
          (status = StatusCode::NO_CONTENT, description = "Block deleted successfully"),
          (status = StatusCode::NOT_FOUND, description = "Block not found"),
          (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn delete_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, HandlerError> {
    state
        .repos
        .blocks
        .delete_by_id(id)
        .await
        .map_err(|e| match e {
            BlockRepositoryError::NotFound { .. } => HandlerError::NotFound,
            _ => {
                error!("Failed to delete block {id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
