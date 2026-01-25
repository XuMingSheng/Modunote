mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::Database;
use storage::repositories::block_related_link_repository::{BlockRelatedLinkResult, test_utils};
use storage_sqlite::SqliteDb;
use storage_sqlite::repositories::{SqliteBlockRelatedLinkRepository, SqliteBlockRepository};

#[rstest]
#[tokio::test]
async fn block_related_link_create_get_delete(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_create_get_delete(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_by_block_ids(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_delete_by_block_ids(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_missing(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_delete_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_delete_by_blocks_missing(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_delete_by_blocks_missing(&link_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_self_link(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_self_link(&link_repo, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_related_link_duplicate(
    #[future] sqlite_db: SqliteDb,
) -> BlockRelatedLinkResult<()> {
    let db = sqlite_db.await;
    let block_repo = SqliteBlockRepository::new();
    let link_repo = SqliteBlockRelatedLinkRepository::new();

    test_utils::assert_duplicate(&link_repo, &block_repo, db.pool()).await
}
