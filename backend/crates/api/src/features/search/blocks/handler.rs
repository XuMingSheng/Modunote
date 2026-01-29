use std::sync::Arc;

use axum::{Json, extract::State};
use tracing::instrument;

use super::{error::SearchBlocksError, request::BlockSearchRequest, response::BlockSearchResponse};
use crate::AppState;
use storage::Database;
use storage::query_services::BlockQueryService;

#[utoipa::path(
    post,
    path = "/api/search/blocks",
    tag = "search",
    responses(
        (status = 200, description = "Success", body = BlockSearchResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn search_blocks(
    State(state): State<Arc<AppState>>,
    Json(request): Json<BlockSearchRequest>,
) -> Result<BlockSearchResponse, SearchBlocksError> {
    let blocks = state
        .query_services
        .blocks
        .search(&request.query, state.db.pool())
        .await?;

    let response: BlockSearchResponse = blocks.into();

    Ok(response)
}
