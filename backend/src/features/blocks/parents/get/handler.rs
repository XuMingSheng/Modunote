use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::response::{GetParentBlocksResponse, ParentBlock};
use storage::{BlockDirectionalLinkRepositoryTrait, BlockRepositoryTrait, RepositoryProvider};

#[utoipa::path(
    get,
    path = "/api/blocks/{id}/parents",
    tag = "blocks",
    params(
        ("id" = uuid::Uuid, Path, description = "Block ID")
    ),
    responses(
        (status = 200, description = "List of parent blocks", body = GetParentBlocksResponse),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn get_parent_blocks(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetParentBlocksResponse>, StatusCode> {
    repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let parent_summaries = repos
        .block_directional_links
        .get_parents_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get parents of block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let parent_blocks = parent_summaries
        .into_iter()
        .map(|summary| ParentBlock {
            block_id: summary.id,
            title: summary.title,
        })
        .collect();

    let response = GetParentBlocksResponse { parent_blocks };
    Ok(Json(response))
}
