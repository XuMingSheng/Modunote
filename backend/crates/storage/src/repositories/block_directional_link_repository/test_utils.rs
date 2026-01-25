use uuid::Uuid;

use domain::blocks::Block;
use sqlx::{Acquire, Database, Executor};

use crate::repositories::block_repository::BlockRepository;

use super::{
    BlockDirectionalLinkRepositoryError, BlockDirectionalLinkRepositoryResult as Result,
    CreateBlockDirectionalLinkDto, traits::BlockDirectionalLinkRepository,
};

async fn seed_block<'e, E, R, DB>(repo: &R, title: &str, executor: E) -> Block
where
    DB: Database,
    R: BlockRepository<DB>,
    E: Executor<'e, Database = DB>,
{
    let block = Block::new(title, "content");
    repo.save(&block, executor)
        .await
        .expect("failed to seed block for link tests");
    block
}

pub async fn assert_create_get_delete<'a, A, L, B, DB>(
    link_repo: &L,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let from = seed_block(block_repo, "from", &mut *tx).await;
    let to = seed_block(block_repo, "to", &mut *tx).await;

    let link_id = Uuid::new_v4();
    let input = CreateBlockDirectionalLinkDto {
        id: link_id,
        block_from_id: from.id,
        block_to_id: to.id,
    };

    let created = link_repo.create(&input, &mut *tx).await?;
    assert_eq!(created.id, link_id);
    assert_eq!(created.block_from_id, from.id);
    assert_eq!(created.block_to_id, to.id);

    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    let fetched = fetched.expect("created link should be retrievable");
    assert_eq!(fetched.id, link_id);

    link_repo.delete_by_id(link_id, &mut *tx).await?;
    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    assert!(fetched.is_none());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_by_block_ids<'e, A, L, B, DB>(
    link_repo: &L,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let from = seed_block(block_repo, "from", &mut *tx).await;
    let to = seed_block(block_repo, "to", &mut *tx).await;

    let link_id = Uuid::new_v4();
    let input = CreateBlockDirectionalLinkDto {
        id: link_id,
        block_from_id: from.id,
        block_to_id: to.id,
    };
    link_repo.create(&input, &mut *tx).await?;

    link_repo
        .delete_by_block_ids(from.id, to.id, &mut *tx)
        .await?;
    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    assert!(fetched.is_none());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_missing<'e, A, L, DB>(link_repo: &L, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let missing_id = Uuid::new_v4();
    let err = link_repo
        .delete_by_id(missing_id, &mut *tx)
        .await
        .expect_err("missing delete should error");

    match err {
        BlockDirectionalLinkRepositoryError::NotFoundById { id } => {
            assert_eq!(id, missing_id);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_by_blocks_missing<'e, A, L, DB>(link_repo: &L, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let from = Uuid::new_v4();
    let to = Uuid::new_v4();

    let err = link_repo
        .delete_by_block_ids(from, to, &mut *tx)
        .await
        .expect_err("missing delete should error");

    match err {
        BlockDirectionalLinkRepositoryError::NotFoundByBlocks {
            from: err_from,
            to: err_to,
        } => {
            assert_eq!(err_from, from);
            assert_eq!(err_to, to);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_cycle_detected<'e, A, L, B, DB>(
    link_repo: &L,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;

    let link_id = Uuid::new_v4();
    let input = CreateBlockDirectionalLinkDto {
        id: link_id,
        block_from_id: a.id,
        block_to_id: b.id,
    };
    link_repo.create(&input, &mut *tx).await?;

    let cycle_input = CreateBlockDirectionalLinkDto {
        id: Uuid::new_v4(),
        block_from_id: b.id,
        block_to_id: a.id,
    };

    let err = link_repo
        .create(&cycle_input, &mut *tx)
        .await
        .expect_err("cycle should error");

    match err {
        BlockDirectionalLinkRepositoryError::CycleDetected { from, to } => {
            assert_eq!(from, b.id);
            assert_eq!(to, a.id);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_duplicate<'e, A, L, B, DB>(link_repo: &L, block_repo: &B, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockDirectionalLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let from = seed_block(block_repo, "from", &mut *tx).await;
    let to = seed_block(block_repo, "to", &mut *tx).await;

    let input = CreateBlockDirectionalLinkDto {
        id: Uuid::new_v4(),
        block_from_id: from.id,
        block_to_id: to.id,
    };
    link_repo.create(&input, &mut *tx).await?;

    let duplicate_input = CreateBlockDirectionalLinkDto {
        id: Uuid::new_v4(),
        block_from_id: from.id,
        block_to_id: to.id,
    };

    let err = link_repo
        .create(&duplicate_input, &mut *tx)
        .await
        .expect_err("duplicate should error");

    match err {
        BlockDirectionalLinkRepositoryError::AlreadyExists {
            from: err_from,
            to: err_to,
        } => {
            assert_eq!(err_from, from.id);
            assert_eq!(err_to, to.id);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}
