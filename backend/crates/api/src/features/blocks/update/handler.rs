use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use super::{error::UpdateBlockError, request::UpdateBlockRequest, response::UpdateBlockResponse};
use crate::AppState;
use storage::{Database, repositories::BlockRepository};

#[utoipa::path(
      put,
      path = "/api/blocks/{id}",
      tag = "blocks",
      request_body = UpdateBlockRequest,
    responses(
        (status = 200, description = "Block updated successfully", body = UpdateBlockResponse),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn update_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateBlockRequest>,
) -> Result<UpdateBlockResponse, UpdateBlockError> {
    let mut block = state
        .repos
        .blocks
        .get_by_id(id, state.db.pool())
        .await?
        .ok_or(UpdateBlockError::NotFound)?;

    if let Some(title) = request.title {
        block.title = title;
    }
    if let Some(content) = request.content {
        block.content = content;
    }
    block.updated_at = Utc::now();

    state.repos.blocks.save(&block, state.db.pool()).await?;

    let response: UpdateBlockResponse = block.into();

    Ok(response)
}
