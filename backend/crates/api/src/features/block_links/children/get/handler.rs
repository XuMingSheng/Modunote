use std::sync::Arc;

use axum::extract::{Path, State};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::{
    error::{ErrorResponse, GetBlockChildLinksError},
    response::GetBlockChildLinksResponse,
};
use crate::AppState;
use storage::{Database, query_services::BlockLinkQueryService};

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/children",
    tag = "block_links",
    responses(
        (status = 200, description = "List of child blocks", body = GetBlockChildLinksResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_child_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<GetBlockChildLinksResponse, GetBlockChildLinksError> {
    let links = state
        .query_services
        .block_links
        .get_child_blocks(id, state.db.pool())
        .await?;

    let response = GetBlockChildLinksResponse {
        child_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok(response)
}
