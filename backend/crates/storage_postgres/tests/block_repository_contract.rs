mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::Database;
use storage::repositories::block_repository::{
    BlockRepostoryResult,
    test_utils::{assert_delete_missing, assert_get_delete_save},
};
use storage_postgres::repositories::PostgresBlockRepository;

#[rstest]
#[tokio::test]
async fn block_repository_get_delete_save(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresBlockRepository::new();
    let mut tx = db.pool().begin().await?;
    let result = assert_get_delete_save(&repo, &mut tx).await;
    let _ = tx.rollback().await;
    result
}

#[rstest]
#[tokio::test]
async fn block_repository_delete_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresBlockRepository::new();
    let mut tx = db.pool().begin().await?;
    let result = assert_delete_missing(&repo, &mut tx).await;
    let _ = tx.rollback().await;
    result
}
