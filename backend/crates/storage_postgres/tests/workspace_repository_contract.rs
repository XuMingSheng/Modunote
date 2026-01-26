mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::Database;
use storage::repositories::workspace_repository::{WorkspaceRepostoryResult, test_utils};
use storage_postgres::repositories::{PostgresBlockRepository, PostgresWorkspaceRepository};

#[rstest]
#[tokio::test]
async fn workspace_get_empty(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> WorkspaceRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresWorkspaceRepository::new();

    test_utils::assert_get_empty(&repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn workspace_save_get_overwrite(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> WorkspaceRepostoryResult<()> {
    let db = postgres_db.await;
    let workspace_repo = PostgresWorkspaceRepository::new();
    let block_repo = PostgresBlockRepository::new();

    test_utils::assert_save_get_overwrite(&workspace_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn workspace_save_missing_blocks(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> WorkspaceRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresWorkspaceRepository::new();

    test_utils::assert_save_missing_blocks(&repo, db.pool()).await
}
