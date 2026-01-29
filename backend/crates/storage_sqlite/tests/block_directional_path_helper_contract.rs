mod fixtures;

use rstest::rstest;

use fixtures::sqlite_db;
use storage::database::Database;
use storage::helpers::block_directional_path_helper::{
    BlockDirectionalPathHelperResult, test_utils,
};
use storage_sqlite::SqliteDb;
use storage_sqlite::helpers::SqliteBlockDirectionalPathHelper;
use storage_sqlite::repositories::SqliteBlockRepository;

#[rstest]
#[tokio::test]
async fn block_directional_path_create_transitive(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = sqlite_db.await;
    let helper = SqliteBlockDirectionalPathHelper::new();
    let block_repo = SqliteBlockRepository::new();

    test_utils::assert_create_paths_for_link_transitive(&helper, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_path_delete_by_link(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = sqlite_db.await;
    let helper = SqliteBlockDirectionalPathHelper::new();
    let block_repo = SqliteBlockRepository::new();

    test_utils::assert_delete_paths_using_link(&helper, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_path_delete_by_block(
    #[future] sqlite_db: SqliteDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = sqlite_db.await;
    let helper = SqliteBlockDirectionalPathHelper::new();
    let block_repo = SqliteBlockRepository::new();

    test_utils::assert_delete_paths_using_block(&helper, &block_repo, db.pool()).await
}
