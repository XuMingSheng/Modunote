mod fixtures;

use rstest::rstest;

use fixtures::postgres_db;
use storage::Database;
use storage::repositories::block_repository::{BlockRepostoryResult, test_utils};
use storage_postgres::repositories::PostgresBlockRepository;

#[rstest]
#[tokio::test]
async fn block_repository_get_delete_save(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresBlockRepository::new();

    test_utils::assert_get_delete_save(&repo, db.pool()).await
}

#[rstest]
#[tokio::test]
async fn block_repository_delete_missing(
    #[future] postgres_db: storage_postgres::PostgresDb,
) -> BlockRepostoryResult<()> {
    let db = postgres_db.await;
    let repo = PostgresBlockRepository::new();

    test_utils::assert_delete_missing(&repo, db.pool()).await
}
