mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::Database;
use storage::repositories::block_related_link_repository::{
    BlockRelatedLinkResult, test_utils,
};
use storage_postgres::repositories::{PostgresBlockRelatedLinkRepository, PostgresBlockRepository};

#[rstest]
#[tokio::test]
async fn block_related_link_create_get_delete(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_create_get_delete(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_by_block_ids(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_delete_by_block_ids(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_delete_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_by_blocks_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_delete_by_blocks_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_self_link(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_self_link(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_duplicate(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRelatedLinkResult<()> {
    let db = postgres_db.await;
    let block_repo = PostgresBlockRepository::new();
    let link_repo = PostgresBlockRelatedLinkRepository::new();

    test_utils::assert_duplicate(&link_repo, &block_repo, db.pool()).await
}
