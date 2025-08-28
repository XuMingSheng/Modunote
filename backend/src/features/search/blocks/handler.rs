use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::{BlockSearchRequest, BlockSearchResponse, BlockSearchResponseItem};
use storage::{BlockRepositoryTrait, RepositoryProvider};

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
    State(repos): State<RepositoryProvider>,
    Json(request): Json<BlockSearchRequest>,
) -> Result<Json<BlockSearchResponse>, StatusCode> {
    let blocks = repos
        .blocks
        .search(&request.query)
        .await
        .map_err(|e| {
            error!("Failed to search blocks by '{}': {e}", request.query);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(|b| BlockSearchResponseItem {
            id: b.id,
            title: b.title,
        })
        .collect();

    let response = BlockSearchResponse { blocks };

    Ok(Json(response))
}
