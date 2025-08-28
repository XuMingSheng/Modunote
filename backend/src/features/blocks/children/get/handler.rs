use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::response::{ChildBlock, GetChildBlocksResponse};
use storage::{BlockDirectionalLinkRepositoryTrait, BlockRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      get,
      path = "/api/blocks/{id}/children",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID")
      ),
      responses(
          (status = 200, description = "List of child blocks", body = GetChildBlocksResponse),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn get_child_blocks(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetChildBlocksResponse>, StatusCode> {
    repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let child_summaries = repos
        .block_directional_links
        .get_children_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get children of block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let child_blocks = child_summaries
        .into_iter()
        .map(|summary| ChildBlock {
            block_id: summary.id,
            title: summary.title,
        })
        .collect();

    let response = GetChildBlocksResponse { child_blocks };
    Ok(Json(response))
}
