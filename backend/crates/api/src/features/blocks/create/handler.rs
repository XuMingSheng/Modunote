use std::sync::Arc;

use axum::{Json, extract::State};
use tracing::instrument;
use utoipa;

use super::{error::CreateBlockError, request::CreateBlockRequest, response::CreateBlockResponse};
use crate::AppState;
use domain::blocks::Block;
use storage::{Database, repositories::BlockRepository};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks",
    tag = "blocks",
    responses(
        (status = 201, description = "Block created successfully", body = CreateBlockResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_block(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBlockRequest>,
) -> Result<CreateBlockResponse, CreateBlockError> {
    let block: Block = request.into();
    state.repos.blocks.save(&block, state.db.pool()).await?;

    let response: CreateBlockResponse = block.into();

    Ok(response)
}
