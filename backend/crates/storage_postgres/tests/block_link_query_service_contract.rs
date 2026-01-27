mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::database::Database;
use storage::query_services::block_link_query_service::BlockLinkQueryServiceResult;
use storage::query_services::block_link_query_service::test_utils::{
    assert_get_child_blocks, assert_get_linked_blocks, assert_get_parent_blocks,
    assert_get_related_blocks,
};
use storage_postgres::query_services::PostgresBlockLinkQueryService;
use storage_postgres::repositories::{
    PostgresBlockDirectionalLinkRepository, PostgresBlockRelatedLinkRepository,
    PostgresBlockRepository,
};
use storage_postgres::PostgresDb;

#[rstest]
#[tokio::test]
async fn block_link_query_service_get_parent_blocks(
    #[future] postgres_db: PostgresDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockLinkQueryService::new();
    let block_repo = PostgresBlockRepository::new();
    let directional_repo = PostgresBlockDirectionalLinkRepository::new();
    let related_repo = PostgresBlockRelatedLinkRepository::new();

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
    #[future] postgres_db: PostgresDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockLinkQueryService::new();
    let block_repo = PostgresBlockRepository::new();
    let directional_repo = PostgresBlockDirectionalLinkRepository::new();
    let related_repo = PostgresBlockRelatedLinkRepository::new();

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
    #[future] postgres_db: PostgresDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockLinkQueryService::new();
    let block_repo = PostgresBlockRepository::new();
    let directional_repo = PostgresBlockDirectionalLinkRepository::new();
    let related_repo = PostgresBlockRelatedLinkRepository::new();

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
    #[future] postgres_db: PostgresDb,
) -> BlockLinkQueryServiceResult<()> {
    let db = postgres_db.await;
    let query_service = PostgresBlockLinkQueryService::new();
    let block_repo = PostgresBlockRepository::new();
    let directional_repo = PostgresBlockDirectionalLinkRepository::new();
    let related_repo = PostgresBlockRelatedLinkRepository::new();

    assert_get_linked_blocks(
        &query_service,
        &block_repo,
        &directional_repo,
        &related_repo,
        db.pool(),
    )
    .await
}
