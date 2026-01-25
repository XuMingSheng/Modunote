mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::Database;
use storage::repositories::block_directional_link_repository::{
    BlockDirectionalLinkRepositoryResult, test_utils,
};
use storage_postgres::repositories::{
    PostgresBlockDirectionalLinkRepository, PostgresBlockRepository,
};

#[rstest]
#[tokio::test]
async fn block_directional_link_create_get_delete(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_create_get_delete(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_by_block_ids(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_by_block_ids(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_by_blocks_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_by_blocks_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_cycle_detected(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_cycle_detected(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_duplicate(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockDirectionalLinkRepository::new();

    test_utils::assert_duplicate(&link_repo, &block_repo, db.pool()).await
}
