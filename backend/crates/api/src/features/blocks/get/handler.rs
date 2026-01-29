use std::sync::Arc;

use axum::extract::{Path, State};
use tracing::instrument;
use uuid::Uuid;

use super::{error::GetBlockError, response::GetBlockResponse};
use crate::AppState;
use storage::query_services::BlockLinkQueryService;
use storage::{Database, repositories::BlockRepository};

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}",
    tag = "blocks",
    responses(
        (status = 200, description = "Block found", body = GetBlockResponse),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<GetBlockResponse, GetBlockError> {
    let block = state
        .repos
        .blocks
        .get_by_id(id, state.db.pool())
        .await?
        .ok_or(GetBlockError::NotFound)?;

    let linked_blocks = state
        .query_services
        .block_links
        .get_linked_blocks(id, state.db.pool())
        .await?;

    let response = GetBlockResponse::from_block_and_linked(block, linked_blocks);

    Ok(response)
}
