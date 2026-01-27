use chrono::{Duration, Utc};
use sqlx::{Acquire, Database, Executor};

use domain::blocks::Block;
use domain::workspaces::workspace::{OpenedBlock, Workspace};

use super::BlockQueryServiceResult as Result;
use crate::query_services::BlockQueryService;
use crate::repositories::{BlockRepository, WorkspaceRepository};

pub async fn assert_get_opened_orders_by_tab_index<'a, A, Q, BR, WR, DB>(
    query_service: &Q,
    block_repo: &BR,
    workspace_repo: &WR,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockQueryService<DB>,
    BR: BlockRepository<DB>,
    WR: WorkspaceRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let block_a = Block::new("block a", "content a");
    let block_b = Block::new("block b", "content b");

    block_repo
        .save(&block_a, &mut *tx)
        .await
        .expect("failed to save block a");
    block_repo
        .save(&block_b, &mut *tx)
        .await
        .expect("failed to save block b");

    let opened_at = Utc::now();
    let workspace = Workspace {
        opened_blocks: vec![
            OpenedBlock {
                block_id: block_a.id,
                opened_at,
                tab_index: 2,
            },
            OpenedBlock {
                block_id: block_b.id,
                opened_at,
                tab_index: 0,
            },
        ],
    };

    workspace_repo
        .save(&workspace, &mut *tx)
        .await
        .expect("failed to save workspace");

    let opened_blocks = query_service.get_opened(&mut *tx).await?;

    assert_eq!(opened_blocks.len(), 2);
    assert_eq!(opened_blocks[0].id, block_b.id);
    assert_eq!(opened_blocks[1].id, block_a.id);
    assert_eq!(opened_blocks[0].tab_index, 0);
    assert_eq!(opened_blocks[1].tab_index, 2);

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_search_matches_title_or_content<'a, A, Q, R, DB>(
    query_service: &Q,
    block_repo: &R,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockQueryService<DB>,
    R: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let base_time = Utc::now();

    let mut block_title = Block::new("find me", "content a");
    block_title.created_at = base_time;
    block_title.updated_at = base_time;

    let mut block_content = Block::new("other", "please find me too");
    block_content.created_at = base_time + Duration::seconds(5);
    block_content.updated_at = base_time + Duration::seconds(10);

    let block_other = Block::new("unrelated", "no match here");

    block_repo
        .save(&block_title, &mut *tx)
        .await
        .expect("failed to save title block");
    block_repo
        .save(&block_content, &mut *tx)
        .await
        .expect("failed to save content block");
    block_repo
        .save(&block_other, &mut *tx)
        .await
        .expect("failed to save other block");

    let results = query_service.search("find", &mut *tx).await?;

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, block_content.id);
    assert_eq!(results[1].id, block_title.id);

    tx.rollback().await?;

    Ok(())
}
