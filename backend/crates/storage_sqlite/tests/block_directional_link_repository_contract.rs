mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::Database;
use storage::repositories::block_directional_link_repository::{
    BlockDirectionalLinkRepositoryResult, test_utils,
};
use storage_sqlite::SqliteDb;
use storage_sqlite::repositories::{SqliteBlockDirectionalLinkRepository, SqliteBlockRepository};

#[rstest]
#[tokio::test]
async fn block_directional_link_create_get_delete(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_create_get_delete(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_by_block_ids(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_by_block_ids(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_missing(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_delete_by_blocks_missing(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_delete_by_blocks_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_cycle_detected(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_cycle_detected(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_link_duplicate(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalLinkRepositoryResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockDirectionalLinkRepository::new();

    test_utils::assert_duplicate(&link_repo, &block_repo, db.pool()).await
}
