use std::sync::Arc;

use axum::extract::{Path, State};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::{
    error::{ErrorResponse, GetBlockParentLinksError},
    response::GetBlockParentLinksResponse,
};
use crate::{
    AppState,
};
use storage::{Database, query_services::BlockLinkQueryService};

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/parents",
    tag = "block_links",
    responses(
        (status = 200, description = "List of parent blocks", body = GetBlockParentLinksResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_parent_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<GetBlockParentLinksResponse, GetBlockParentLinksError> {
    let links = state
        .query_services
        .block_links
        .get_parent_blocks(id, state.db.pool())
        .await?;

    let response = GetBlockParentLinksResponse {
        parent_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok(response)
}
