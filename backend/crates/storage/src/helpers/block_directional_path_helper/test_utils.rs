use crate::helpers::block_directional_path_helper::{
    BlockDirectionalPathHelper, BlockDirectionalPathHelperResult as Result,
};
use crate::repositories::block_repository::BlockRepository;
use domain::blocks::Block;
use sqlx::{Acquire, Database, Executor};

async fn seed_block<'e, E, R, DB>(repo: &R, title: &str, executor: E) -> Block
where
    DB: Database,
    R: BlockRepository<DB>,
    E: Executor<'e, Database = DB>,
{
    let block = Block::new(title, "content");
    repo.save(&block, executor)
        .await
        .expect("failed to seed block for path helper tests");
    block
}

pub async fn assert_create_paths_for_link_transitive<'e, A, H, R, DB>(
    helper: &H,
    block_repo: &R,
    conn: A,
) -> Result<()>
where
    DB: Database,
    H: BlockDirectionalPathHelper<DB>,
    R: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;
    let c = seed_block(block_repo, "c", &mut *tx).await;

    helper.create_paths_for_link(a.id, b.id, &mut *tx).await?;
    helper.create_paths_for_link(b.id, c.id, &mut *tx).await?;

    assert!(helper.is_ancestor_descendant(a.id, b.id, &mut *tx).await?);
    assert!(helper.is_ancestor_descendant(b.id, c.id, &mut *tx).await?);
    assert!(helper.is_ancestor_descendant(a.id, c.id, &mut *tx).await?);

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_paths_using_link<'e, A, H, R, DB>(
    helper: &H,
    block_repo: &R,
    conn: A,
) -> Result<()>
where
    DB: Database,
    H: BlockDirectionalPathHelper<DB>,
    R: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;
    let c = seed_block(block_repo, "c", &mut *tx).await;
    let d = seed_block(block_repo, "d", &mut *tx).await;

    helper.create_paths_for_link(a.id, b.id, &mut *tx).await?;
    helper.create_paths_for_link(b.id, c.id, &mut *tx).await?;
    helper.create_paths_for_link(c.id, d.id, &mut *tx).await?;

    assert!(helper.is_ancestor_descendant(a.id, c.id, &mut *tx).await?);
    assert!(helper.is_ancestor_descendant(a.id, d.id, &mut *tx).await?);

    helper.delete_paths_using_link(b.id, c.id, &mut *tx).await?;

    assert!(helper.is_ancestor_descendant(a.id, b.id, &mut *tx).await?);
    assert!(helper.is_ancestor_descendant(c.id, d.id, &mut *tx).await?);
    assert!(!helper.is_ancestor_descendant(a.id, c.id, &mut *tx).await?);
    assert!(!helper.is_ancestor_descendant(a.id, d.id, &mut *tx).await?);

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_paths_using_block<'e, A, H, R, DB>(
    helper: &H,
    block_repo: &R,
    conn: A,
) -> Result<()>
where
    DB: Database,
    H: BlockDirectionalPathHelper<DB>,
    R: BlockRepository<DB>,
    A: Acquire<'e, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let a = seed_block(block_repo, "a", &mut *tx).await;
    let b = seed_block(block_repo, "b", &mut *tx).await;
    let c = seed_block(block_repo, "c", &mut *tx).await;

    helper.create_paths_for_link(a.id, b.id, &mut *tx).await?;
    helper.create_paths_for_link(b.id, c.id, &mut *tx).await?;

    assert!(helper.is_ancestor_descendant(a.id, c.id, &mut *tx).await?);

    helper.delete_paths_using_block(b.id, &mut *tx).await?;

    assert!(!helper.is_ancestor_descendant(a.id, c.id, &mut *tx).await?);

    tx.rollback().await?;

    Ok(())
}
