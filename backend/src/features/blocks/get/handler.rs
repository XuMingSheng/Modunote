use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::response::{BlockLink, GetBlockResponse};
use storage::{
    BlockDirectionalLinkRepositoryTrait, BlockRelatedLinkRepositoryTrait, BlockRepositoryTrait,
    BlockSummary, RepositoryProvider,
};

#[utoipa::path(
    get,
    path = "/api/blocks/{id}",
    tag = "blocks",
    params(
        ("id" = uuid::Uuid, Path, description = "Block ID")
    ),
    responses(
        (status = 200, description = "Block found", body = GetBlockResponse),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn get_block(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetBlockResponse>, StatusCode> {
    let block = repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {id}, {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let parents = repos
        .block_directional_links
        .get_parents_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get parents of block {id}, {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(map_block_summary_to_link)
        .collect();

    let children = repos
        .block_directional_links
        .get_children_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get children of block {id}, {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(map_block_summary_to_link)
        .collect();

    let related = repos
        .block_related_links
        .get_related_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get related links of block {id}, {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(map_block_summary_to_link)
        .collect();

    let response = GetBlockResponse {
        id: block.id,
        title: block.title,
        content: block.content,
        parent_blocks: parents,
        child_blocks: children,
        related_blocks: related,
    };

    Ok(Json(response))
}

fn map_block_summary_to_link(summary: BlockSummary) -> BlockLink {
    BlockLink {
        block_id: summary.id,
        title: summary.title,
    }
}
