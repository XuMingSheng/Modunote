use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::{request::UpdateBlockRequest, response::UpdateBlockResponse};
use crate::{AppState, features::error::HandlerError};
use storage::repositories::BlockRepository;
use storage::repositories::block_repository::{BlockRepositoryError, UpdateBlockDto};

#[utoipa::path(
      put,
      path = "/api/blocks/{id}",
      tag = "blocks",
      request_body = UpdateBlockRequest,
      responses(
          (status = StatusCode::OK, description = "Block updated successfully", body = UpdateBlockResponse),
          (status = StatusCode::NOT_FOUND, description = "Block not found"),
          (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn update_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateBlockRequest>,
) -> Result<(StatusCode, Json<UpdateBlockResponse>), HandlerError> {
    let input: UpdateBlockDto = request.into();

    let updated_block = state
        .repos
        .blocks
        .update_by_id(id, &input)
        .await
        .map_err(|e| match e {
            BlockRepositoryError::NotFound { .. } => HandlerError::NotFound,
            _ => {
                error!("Failed to update block {id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    let response: UpdateBlockResponse = updated_block.into();

    Ok((StatusCode::OK, Json(response)))
}
