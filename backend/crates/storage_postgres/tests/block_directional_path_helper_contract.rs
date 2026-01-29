mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::database::Database;
use storage::helpers::block_directional_path_helper::{
    BlockDirectionalPathHelperResult, test_utils,
};
use storage_postgres::PostgresDb;
use storage_postgres::helpers::PostgresBlockDirectionalPathHelper;
use storage_postgres::repositories::PostgresBlockRepository;

#[rstest]
#[tokio::test]
async fn block_directional_path_create_transitive(
    #[future] postgres_db: PostgresDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = postgres_db.await;
    let helper = PostgresBlockDirectionalPathHelper::new();
    let block_repo = PostgresBlockRepository::new();

    test_utils::assert_create_paths_for_link_transitive(&helper, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_path_delete_by_link(
    #[future] postgres_db: PostgresDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = postgres_db.await;
    let helper = PostgresBlockDirectionalPathHelper::new();
    let block_repo = PostgresBlockRepository::new();

    test_utils::assert_delete_paths_using_link(&helper, &block_repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_directional_path_delete_by_block(
    #[future] postgres_db: PostgresDb,
) -> BlockDirectionalPathHelperResult<()> {
    let db = postgres_db.await;
    let helper = PostgresBlockDirectionalPathHelper::new();
    let block_repo = PostgresBlockRepository::new();

    test_utils::assert_delete_paths_using_block(&helper, &block_repo, db.pool()).await
}
