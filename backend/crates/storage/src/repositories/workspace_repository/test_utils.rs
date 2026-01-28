use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use domain::blocks::Block;
use domain::workspaces::{OpenedBlock, Workspace};
use sqlx::{Acquire, Database, Executor};

use crate::repositories::block_repository::BlockRepository;

use super::{WorkspaceRepository, WorkspaceRepositoryError, WorkspaceRepostoryResult as Result};

async fn seed_block<'e, E, R, DB>(repo: &R, title: &str, executor: E) -> Block
where
    DB: Database,
    R: BlockRepository<DB>,
    E: Executor<'e, Database = DB>,
{
    let block = Block::new(title, "content");
    repo.save(&block, executor)
        .await
        .expect("failed to seed block for workspace tests");
    block
}

fn opened_block(block_id: Uuid, tab_index: usize, opened_at: DateTime<Utc>) -> OpenedBlock {
    OpenedBlock {
        block_id,
        opened_at,
        tab_index,
    }
}

pub async fn assert_get_empty<'a, A, R, DB>(repo: &R, conn: A) -> Result<()>
where
    DB: Database,
    R: WorkspaceRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let workspace = repo.get(&mut *tx).await?;
    assert!(workspace.opened_blocks.is_empty());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_save_get_overwrite<'a, A, W, B, DB>(
    workspace_repo: &W,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    W: WorkspaceRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;
    let c = seed_block(block_repo, "c", &mut *tx).await;

    let opened_at_a = Utc::now();
    let opened_at_b = Utc::now();

    let workspace = Workspace {
        opened_blocks: vec![
            opened_block(a.id, 0, opened_at_a),
            opened_block(b.id, 1, opened_at_b),
        ],
    };

    workspace_repo.save(&workspace, &mut *tx).await?;

    let fetched = workspace_repo.get(&mut *tx).await?;
    let fetched_map: HashMap<Uuid, (DateTime<Utc>, usize)> = fetched
        .opened_blocks
        .into_iter()
        .map(|b| (b.block_id, (b.opened_at, b.tab_index)))
        .collect();

    assert_eq!(fetched_map.len(), 2);
    assert_eq!(fetched_map.get(&a.id), Some(&(opened_at_a, 0)));
    assert_eq!(fetched_map.get(&b.id), Some(&(opened_at_b, 1)));

    let opened_at_c = Utc::now();
    let overwrite = Workspace {
        opened_blocks: vec![opened_block(c.id, 2, opened_at_c)],
    };

    workspace_repo.save(&overwrite, &mut *tx).await?;

    let fetched = workspace_repo.get(&mut *tx).await?;
    assert_eq!(fetched.opened_blocks.len(), 1);
    assert_eq!(fetched.opened_blocks[0].block_id, c.id);
    assert_eq!(fetched.opened_blocks[0].tab_index, 2);

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_save_missing_blocks<'a, A, R, DB>(repo: &R, conn: A) -> Result<()>
where
    DB: Database,
    R: WorkspaceRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let missing = Uuid::new_v4();
    let workspace = Workspace {
        opened_blocks: vec![opened_block(missing, 0, Utc::now())],
    };

    let err = repo
        .save(&workspace, &mut *tx)
        .await
        .expect_err("missing blocks should error");

    match err {
        WorkspaceRepositoryError::SomeBlocksNotFound => {}
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}
