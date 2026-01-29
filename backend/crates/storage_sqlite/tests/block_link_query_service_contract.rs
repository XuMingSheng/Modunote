mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::database::Database;
use storage::query_services::block_link_query_service::BlockLinkQueryServiceResult;
use storage::query_services::block_link_query_service::test_utils::{
    assert_get_child_blocks, assert_get_linked_blocks, assert_get_parent_blocks,
    assert_get_related_blocks,
};
use storage_sqlite::query_services::SqliteBlockLinkQueryService;
use storage_sqlite::repositories::{
    SqliteBlockDirectionalLinkRepository, SqliteBlockRelatedLinkRepository, SqliteBlockRepository,
};
use storage_sqlite::SqliteDb;

#[rstest]
#[tokio::test]
async fn block_link_query_service_get_parent_blocks(
    #[future] sqlite_db: SqliteDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockLinkQueryService::new();
    let block_repo = SqliteBlockRepository::new();
    let directional_repo = SqliteBlockDirectionalLinkRepository::new();
    let related_repo = SqliteBlockRelatedLinkRepository::new();

    assert_get_parent_blocks(
        &query_service,
        &block_repo,
        &directional_repo,
        &related_repo,
        db.pool(),
    )
    .await
}

#[rstest]
#[tokio::test]
async fn block_link_query_service_get_child_blocks(
    #[future] sqlite_db: SqliteDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockLinkQueryService::new();
    let block_repo = SqliteBlockRepository::new();
    let directional_repo = SqliteBlockDirectionalLinkRepository::new();
    let related_repo = SqliteBlockRelatedLinkRepository::new();

    assert_get_child_blocks(
        &query_service,
        &block_repo,
        &directional_repo,
        &related_repo,
        db.pool(),
    )
    .await
}

#[rstest]
#[tokio::test]
async fn block_link_query_service_get_related_blocks(
    #[future] sqlite_db: SqliteDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockLinkQueryService::new();
    let block_repo = SqliteBlockRepository::new();
    let directional_repo = SqliteBlockDirectionalLinkRepository::new();
    let related_repo = SqliteBlockRelatedLinkRepository::new();

    assert_get_related_blocks(
        &query_service,
        &block_repo,
        &directional_repo,
        &related_repo,
        db.pool(),
    )
    .await
}

#[rstest]
#[tokio::test]
async fn block_link_query_service_get_linked_blocks(
    #[future] sqlite_db: SqliteDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = sqlite_db.await;
    let query_service = SqliteBlockLinkQueryService::new();
    let block_repo = SqliteBlockRepository::new();
    let directional_repo = SqliteBlockDirectionalLinkRepository::new();
    let related_repo = SqliteBlockRelatedLinkRepository::new();

    assert_get_linked_blocks(
        &query_service,
        &block_repo,
        &directional_repo,
        &related_repo,
        db.pool(),
    )
    .await
}
