use std::sync::Arc;

use axum::extract::{Path, State};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::{
    error::{ErrorResponse, GetBlockRelatedLinksError},
    response::GetBlockRelatedLinksResponse,
};
use crate::AppState;
use storage::{Database, query_services::BlockLinkQueryService};

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/related",
    tag = "block_links",
    responses(
        (status = 200, description = "List of related blocks", body = GetBlockRelatedLinksResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_related_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<GetBlockRelatedLinksResponse, GetBlockRelatedLinksError> {
    let links = state
        .query_services
        .block_links
        .get_related_blocks(id, state.db.pool())
        .await?;

    let response = GetBlockRelatedLinksResponse {
        related_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok(response)
}
