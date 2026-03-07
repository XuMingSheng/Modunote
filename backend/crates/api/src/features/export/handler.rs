use std::io::{Cursor, Write};
use std::sync::Arc;

use axum::extract::State;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use super::{error::ExportError, response::ExportResponse};
use crate::AppState;
use storage::query_services::{BlockLinkQueryService, BlockQueryService};
use storage::Database;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BlockRecord {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/export",
    tag = "export",
    responses(
        (status = 200, description = "ZIP archive of all blocks and relations", content_type = "application/zip"),
        (status = 500, description = "Internal server error"),
    )
)]
#[instrument(err, skip(state))]
pub async fn export(
    State(state): State<Arc<AppState>>,
) -> Result<ExportResponse, ExportError> {
    let pool = state.db.pool();

    let blocks = state.query_services.blocks.get_all(pool).await?;
    let directional = state.query_services.block_links.get_all_directional(pool).await?;
    let related = state.query_services.block_links.get_all_related(pool).await?;

    let cursor = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("blocks.jsonl", options)?;
    for block in &blocks {
        let record = BlockRecord {
            id: block.id,
            title: block.title.clone(),
            content: block.content.clone(),
            created_at: block.created_at,
            updated_at: block.updated_at,
        };
        let line = serde_json::to_string(&record)?;
        zip.write_all(line.as_bytes())?;
        zip.write_all(b"\n")?;
    }

    zip.start_file("directional_links.json", options)?;
    let dir_pairs: Vec<[Uuid; 2]> = directional
        .iter()
        .map(|l| [l.block_from_id, l.block_to_id])
        .collect();
    zip.write_all(serde_json::to_string(&dir_pairs)?.as_bytes())?;

    zip.start_file("related_links.json", options)?;
    let rel_pairs: Vec<[Uuid; 2]> = related
        .iter()
        .map(|l| [l.block_a_id, l.block_b_id])
        .collect();
    zip.write_all(serde_json::to_string(&rel_pairs)?.as_bytes())?;

    let bytes = zip.finish()?.into_inner();

    ExportResponse::new(bytes)
}
