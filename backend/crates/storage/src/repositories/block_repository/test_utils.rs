use uuid::Uuid;

use sqlx::{Acquire, Database, Executor};

use super::{
    error::BlockRepositoryError, error::BlockRepostoryResult as Result, traits::BlockRepository,
};
use domain::blocks::Block;

pub async fn assert_get_delete_save<'a, A, R, DB>(repo: &R, conn: A) -> Result<()>
where
    DB: Database,
    R: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let mut created_block = Block::new("Title", "Content");
    let block_id = created_block.id;

    repo.save(&created_block, &mut *tx).await?;

    let fetched = repo.get_by_id(block_id, &mut *tx).await?;
    let fetched = fetched.expect("created block should be retrievable");
    assert_eq!(fetched.id, created_block.id);
    assert_eq!(fetched.title, created_block.title);
    assert_eq!(fetched.content, created_block.content);

    created_block.title = "New title".to_string();
    created_block.content = "New content".to_string();
    repo.save(&created_block, &mut *tx).await?;

    let fetched = repo.get_by_id(block_id, &mut *tx).await?;
    let fetched = fetched.expect("updated block should be retrievable");
    assert_eq!(fetched.title, created_block.title);
    assert_eq!(fetched.content, created_block.content);

    repo.delete_by_id(block_id, &mut *tx).await?;
    let fetched = repo.get_by_id(block_id, &mut *tx).await?;
    assert!(fetched.is_none());

    tx.rollback().await?;

    Ok(())
}

pub async fn assert_delete_missing<'a, A, R, DB>(repo: &R, conn: A) -> Result<()>
where
    DB: Database,
    R: BlockRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let missing_id = Uuid::new_v4();
    let err = repo
        .delete_by_id(missing_id, &mut *tx)
        .await
        .expect_err("deleting a missing block should error");

    match err {
        BlockRepositoryError::NotFound { id } => {
            assert_eq!(id, missing_id);
        }
        other => {
            return Err(other);
        }
    }

    tx.rollback().await?;

    Ok(())
}
