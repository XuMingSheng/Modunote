mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::database::Database;
use storage::repositories::block_repository::BlockRepostoryResult;
use storage::repositories::block_repository::test_utils::{
    assert_delete_missing, assert_get_delete_save,
};
use storage_sqlite::SqliteDb;
use storage_sqlite::repositories::SqliteBlockRepository;

#[rstest]
#[tokio::test]
async fn block_repository_get_delete_save(
    #[future] sqlite_db: SqliteDb,
) -> BlockRepostoryResult<()> {
    let db = sqlite_db.await;
    let repo = SqliteBlockRepository::new();
    assert_get_delete_save(&repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_repository_delete_missing(
    #[future] sqlite_db: SqliteDb,
) -> BlockRepostoryResult<()> {
    let db = sqlite_db.await;
    let repo = SqliteBlockRepository::new();
    let mut tx = db.pool().begin().await?;
    let result = assert_delete_missing(&repo, &mut tx).await;
    let _ = tx.rollback().await;
    result
}
