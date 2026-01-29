use uuid::Uuid;

use domain::blocks::Block;
use sqlx::{Acquire, Database, Executor};

use crate::repositories::block_repository::BlockRepository;

use super::{
    BlockRelatedLinkError, BlockRelatedLinkResult as Result, CreateBlockRelatedLinkDto,
    traits::BlockRelatedLinkRepository,
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
        .expect("failed to seed block for related link tests");
    block
}

fn ordered_ids(block_a_id: Uuid, block_b_id: Uuid) -> (Uuid, Uuid) {
    if block_a_id > block_b_id {
        (block_b_id, block_a_id)
    } else {
        (block_a_id, block_b_id)
    }
}

pub async fn assert_create_get_delete<'a, A, L, B, DB>(
    link_repo: &L,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;

    let link_id = Uuid::new_v4();
    let input = CreateBlockRelatedLinkDto {
        id: link_id,
        block_a_id: a.id,
        block_b_id: b.id,
    };

    let created = link_repo.create(&input, &mut *tx).await?;
    let (expected_a, expected_b) = ordered_ids(a.id, b.id);
    assert_eq!(created.id, link_id);
    assert_eq!(created.block_a_id, expected_a);
    assert_eq!(created.block_b_id, expected_b);

    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    let fetched = fetched.expect("created related link should be retrievable");
    assert_eq!(fetched.id, link_id);

    link_repo.delete_by_id(link_id, &mut *tx).await?;
    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    assert!(fetched.is_none());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_by_block_ids<'a, A, L, B, DB>(
    link_repo: &L,
    block_repo: &B,
    conn: A,
) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;

    let link_id = Uuid::new_v4();
    let input = CreateBlockRelatedLinkDto {
        id: link_id,
        block_a_id: a.id,
        block_b_id: b.id,
    };
    link_repo.create(&input, &mut *tx).await?;

    link_repo.delete_by_block_ids(a.id, b.id, &mut *tx).await?;
    let fetched = link_repo.get_by_id(link_id, &mut *tx).await?;
    assert!(fetched.is_none());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_missing<'a, A, L, DB>(link_repo: &L, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
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
        BlockRelatedLinkError::NotFoundById { id } => {
            assert_eq!(id, missing_id);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_by_blocks_missing<'a, A, L, DB>(link_repo: &L, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = Uuid::new_v4();
    let b = Uuid::new_v4();

    let err = link_repo
        .delete_by_block_ids(a, b, &mut *tx)
        .await
        .expect_err("missing delete should error");

    match err {
        BlockRelatedLinkError::NotFoundByBlocks { a: err_a, b: err_b } => {
            let (expected_a, expected_b) = ordered_ids(a, b);
            assert_eq!(err_a, expected_a);
            assert_eq!(err_b, expected_b);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_self_link<'a, A, L, B, DB>(link_repo: &L, block_repo: &B, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let block = seed_block(block_repo, "self", &mut *tx).await;
    let input = CreateBlockRelatedLinkDto {
        id: Uuid::new_v4(),
        block_a_id: block.id,
        block_b_id: block.id,
    };

    let err = link_repo
        .create(&input, &mut *tx)
        .await
        .expect_err("self link should error");

    match err {
        BlockRelatedLinkError::SelfLink { id } => {
            assert_eq!(id, block.id);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_duplicate<'a, A, L, B, DB>(link_repo: &L, block_repo: &B, conn: A) -> Result<()>
where
    DB: Database,
    L: BlockRelatedLinkRepository<DB>,
    B: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;

    let input = CreateBlockRelatedLinkDto {
        id: Uuid::new_v4(),
        block_a_id: a.id,
        block_b_id: b.id,
    };
    link_repo.create(&input, &mut *tx).await?;

    let duplicate = CreateBlockRelatedLinkDto {
        id: Uuid::new_v4(),
        block_a_id: b.id,
        block_b_id: a.id,
    };

    let err = link_repo
        .create(&duplicate, &mut *tx)
        .await
        .expect_err("duplicate link should error");

    match err {
        BlockRelatedLinkError::AlreadyExists { a: err_a, b: err_b } => {
            let (expected_a, expected_b) = ordered_ids(a.id, b.id);
            assert_eq!(err_a, expected_a);
            assert_eq!(err_b, expected_b);
        }
        other => return Err(other),
    }

    tx.rollback().await?;

    Ok(())
}
