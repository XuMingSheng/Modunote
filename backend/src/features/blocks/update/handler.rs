use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::{request::UpdateBlockRequest, response::UpdateBlockResponse};
use storage::{BlockRepositoryTrait, BlockUpdate, RepositoryProvider};

#[utoipa::path(
      put,
      path = "/api/blocks/{id}",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID")
      ),
      request_body = UpdateBlockRequest,
      responses(
          (status = 200, description = "Block updated successfully", body = UpdateBlockResponse),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn update_block(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateBlockRequest>,
) -> Result<Json<UpdateBlockResponse>, StatusCode> {
    repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {id}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let block_update = BlockUpdate {
        title: request.title,
        content: request.content,
    };

    let updated_block = repos
        .blocks
        .update_by_id(id, &block_update)
        .await
        .map_err(|e| {
            error!("Failed to update block {id}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = UpdateBlockResponse {
        id: updated_block.id,
        title: updated_block.title,
        content: updated_block.content,
        updated_at: updated_block.updated_at,
    };

    Ok(Json(response))
}
