use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Read};
use std::sync::Arc;

use axum::extract::{Multipart, State};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;
use zip::ZipArchive;

use super::{error::ImportError, response::ImportResponse};
use crate::AppState;
use domain::blocks::Block;
use storage::Database;
use storage::query_services::block_query_service::BlockExportDto;
use storage::query_services::{BlockLinkQueryService, BlockQueryService};
use storage::repositories::block_directional_link_repository::{
    BlockDirectionalLinkRepositoryError, CreateBlockDirectionalLinkDto,
};
use storage::repositories::block_related_link_repository::{
    BlockRelatedLinkError, CreateBlockRelatedLinkDto,
};
use storage::repositories::{
    BlockDirectionalLinkRepository, BlockRelatedLinkRepository, BlockRepository,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportedBlock {
    id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

struct BlockImportResult {
    inserted: usize,
    updated: usize,
    skipped: usize,
    new_ids: HashSet<Uuid>,
}

struct LinkImportResult {
    inserted: usize,
    skipped: usize,
}

#[utoipa::path(
    post,
    path = "/api/import",
    tag = "import",
    responses(
        (status = 200, description = "Import summary", body = ImportResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
#[instrument(err, skip(state, multipart))]
pub async fn import(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<ImportResponse, ImportError> {
    let zip_bytes = loop {
        match multipart.next_field().await? {
            None => return Err(ImportError::MissingFile),
            Some(field) if field.name() == Some("file") => {
                break field.bytes().await?.to_vec();
            }
            Some(_) => continue,
        }
    };

    let pool = state.db.pool();

    let existing_blocks: HashMap<Uuid, _> = state
        .query_services
        .blocks
        .get_all(pool)
        .await?
        .into_iter()
        .map(|b| (b.id, b))
        .collect();

    let mut known_block_ids: HashSet<Uuid> = existing_blocks.keys().copied().collect();

    let dir_set: HashSet<(Uuid, Uuid)> = state
        .query_services
        .block_links
        .get_all_directional(pool)
        .await?
        .into_iter()
        .map(|l| (l.block_from_id, l.block_to_id))
        .collect();

    let rel_set: HashSet<(Uuid, Uuid)> = state
        .query_services
        .block_links
        .get_all_related(pool)
        .await?
        .into_iter()
        .map(|l| (l.block_a_id, l.block_b_id))
        .collect();

    let mut archive = ZipArchive::new(Cursor::new(zip_bytes))?;

    let blocks_content = read_zip_entry(&mut archive, "blocks.jsonl")?;
    let blocks = process_blocks(&blocks_content, &existing_blocks, &state).await?;
    known_block_ids.extend(&blocks.new_ids);

    let dir_content = read_zip_entry(&mut archive, "directional_links.json")?;
    let dir_links =
        process_directional_links(&dir_content, &dir_set, &known_block_ids, &state).await?;

    let rel_content = read_zip_entry(&mut archive, "related_links.json")?;
    let rel_links = process_related_links(&rel_content, &rel_set, &known_block_ids, &state).await?;

    Ok(ImportResponse {
        blocks_inserted: blocks.inserted,
        blocks_updated: blocks.updated,
        blocks_skipped: blocks.skipped,
        dir_links_inserted: dir_links.inserted,
        dir_links_skipped: dir_links.skipped,
        related_links_inserted: rel_links.inserted,
        related_links_skipped: rel_links.skipped,
    })
}

async fn process_blocks(
    content: &str,
    existing_blocks: &HashMap<Uuid, BlockExportDto>,
    state: &AppState,
) -> Result<BlockImportResult, ImportError> {
    let mut result = BlockImportResult {
        inserted: 0,
        updated: 0,
        skipped: 0,
        new_ids: HashSet::new(),
    };

    for line in content.lines().filter(|l| !l.trim().is_empty()) {
        let imported: ImportedBlock = serde_json::from_str(line)?;

        match existing_blocks.get(&imported.id) {
            None => {
                let block = Block {
                    id: imported.id,
                    title: imported.title,
                    content: imported.content,
                    created_at: imported.created_at,
                    updated_at: imported.updated_at,
                };
                state.repos.blocks.save(&block, state.db.pool()).await?;
                result.new_ids.insert(imported.id);
                result.inserted += 1;
            }
            Some(existing) if imported.updated_at > existing.updated_at => {
                let block = Block {
                    id: imported.id,
                    title: imported.title,
                    content: imported.content,
                    created_at: existing.created_at,
                    updated_at: imported.updated_at,
                };
                state.repos.blocks.save(&block, state.db.pool()).await?;
                result.updated += 1;
            }
            Some(_) => {
                result.skipped += 1;
            }
        }
    }

    Ok(result)
}

async fn process_directional_links(
    content: &str,
    dir_set: &HashSet<(Uuid, Uuid)>,
    known_block_ids: &HashSet<Uuid>,
    state: &AppState,
) -> Result<LinkImportResult, ImportError> {
    let mut result = LinkImportResult {
        inserted: 0,
        skipped: 0,
    };

    let pairs: Vec<[Uuid; 2]> = serde_json::from_str(content)?;
    for [from_id, to_id] in pairs {
        if dir_set.contains(&(from_id, to_id)) {
            result.skipped += 1;
            continue;
        }
        if !known_block_ids.contains(&from_id) || !known_block_ids.contains(&to_id) {
            tracing::warn!(%from_id, %to_id, "Skipping directional link: block not found");
            result.skipped += 1;
            continue;
        }
        let dto = CreateBlockDirectionalLinkDto {
            id: Uuid::new_v4(),
            block_from_id: from_id,
            block_to_id: to_id,
        };
        match state
            .repos
            .block_directional_links
            .create(&dto, state.db.pool())
            .await
        {
            Ok(_) => result.inserted += 1,
            Err(BlockDirectionalLinkRepositoryError::AlreadyExists { .. }) => result.skipped += 1,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(result)
}

async fn process_related_links(
    content: &str,
    rel_set: &HashSet<(Uuid, Uuid)>,
    known_block_ids: &HashSet<Uuid>,
    state: &AppState,
) -> Result<LinkImportResult, ImportError> {
    let mut result = LinkImportResult {
        inserted: 0,
        skipped: 0,
    };

    let pairs: Vec<[Uuid; 2]> = serde_json::from_str(content)?;
    for [a, b] in pairs {
        let (a_id, b_id) = if a < b { (a, b) } else { (b, a) };
        if rel_set.contains(&(a_id, b_id)) {
            result.skipped += 1;
            continue;
        }
        if !known_block_ids.contains(&a_id) || !known_block_ids.contains(&b_id) {
            tracing::warn!(%a_id, %b_id, "Skipping related link: block not found");
            result.skipped += 1;
            continue;
        }
        let dto = CreateBlockRelatedLinkDto {
            id: Uuid::new_v4(),
            block_a_id: a_id,
            block_b_id: b_id,
        };
        match state
            .repos
            .block_related_links
            .create(&dto, state.db.pool())
            .await
        {
            Ok(_) => result.inserted += 1,
            Err(BlockRelatedLinkError::AlreadyExists { .. }) => result.skipped += 1,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(result)
}

fn read_zip_entry(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    name: &str,
) -> Result<String, ImportError> {
    let mut file = archive.by_name(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
