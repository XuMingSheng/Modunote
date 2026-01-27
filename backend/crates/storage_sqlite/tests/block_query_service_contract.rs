mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::database::Database;
use storage::query_services::block_query_service::BlockQueryServiceResult;
use storage::query_services::block_query_service::test_utils::{
    assert_get_opened_orders_by_tab_index, assert_search_matches_title_or_content,
};
use storage_sqlite::query_services::SqliteBlockQueryService;
use storage_sqlite::repositories::{SqliteBlockRepository, SqliteWorkspaceRepository};
use storage_sqlite::SqliteDb;

#[rstest]
#[tokio::test]
async fn block_query_service_get_opened_orders_by_tab_index(
    #[future] sqlite_db: SqliteDb,
) -> BlockQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockQueryService::new();
    let block_repo = SqliteBlockRepository::new();
    let workspace_repo = SqliteWorkspaceRepository::new();

    assert_get_opened_orders_by_tab_index(
        &query_service,
        &block_repo,
        &workspace_repo,
        db.pool(),
    )
    .await
}

#[rstest]
#[tokio::test]
async fn block_query_service_search_matches_title_or_content(
    #[future] sqlite_db: SqliteDb,
) -> BlockQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockQueryService::new();
    let block_repo = SqliteBlockRepository::new();

    assert_search_matches_title_or_content(&query_service, &block_repo, db.pool()).await
}
