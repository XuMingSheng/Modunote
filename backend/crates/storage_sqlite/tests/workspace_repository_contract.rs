mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::Database;
use storage::repositories::workspace_repository::{WorkspaceRepostoryResult, test_utils};
use storage_sqlite::SqliteDb;
use storage_sqlite::repositories::{SqliteBlockRepository, SqliteWorkspaceRepository};

#[rstest]
#[tokio::test]
async fn workspace_get_empty(#[future] sqlite_db: SqliteDb) -> WorkspaceRepostoryResult<()> {
    let db = sqlite_db.await;
    let repo = SqliteWorkspaceRepository::new();

    test_utils::assert_get_empty(&repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn workspace_save_get_overwrite(
    #[future] sqlite_db: SqliteDb,
) -> WorkspaceRepostoryResult<()> {
    let db = sqlite_db.await;
    let workspace_repo = SqliteWorkspaceRepository::new();
    let block_repo = SqliteBlockRepository::new();

    test_utils::assert_save_get_overwrite(&workspace_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn workspace_save_missing_blocks(
    #[future] sqlite_db: SqliteDb,
) -> WorkspaceRepostoryResult<()> {
    let db = sqlite_db.await;
    let repo = SqliteWorkspaceRepository::new();

    test_utils::assert_save_missing_blocks(&repo, db.pool()).await
}
