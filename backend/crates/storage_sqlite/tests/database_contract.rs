use storage::database::{DatabaseResult, test_utils::connect_and_run_migration};

#[tokio::test]
async fn sqlite_database_contract() -> DatabaseResult<()> {
    connect_and_run_migration::<storage_sqlite::SqliteDb>("sqlite::memory:").await
}
