mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::database::Database;
use storage::query_services::block_query_service::BlockQueryServiceResult;
use storage::query_services::block_query_service::test_utils::{
    assert_get_opened_orders_by_tab_index, assert_search_matches_title_or_content,
};
use storage_postgres::query_services::PostgresBlockQueryService;
use storage_postgres::repositories::{PostgresBlockRepository, PostgresWorkspaceRepository};
use storage_postgres::PostgresDb;

#[rstest]
#[tokio::test]
async fn block_query_service_get_opened_orders_by_tab_index(
    #[future] postgres_db: PostgresDb,
) -> BlockQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockQueryService::new();
    let block_repo = PostgresBlockRepository::new();
    let workspace_repo = PostgresWorkspaceRepository::new();

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
    #[future] postgres_db: PostgresDb,
) -> BlockQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockQueryService::new();
    let block_repo = PostgresBlockRepository::new();

    assert_search_matches_title_or_content(&query_service, &block_repo, db.pool()).await
}
