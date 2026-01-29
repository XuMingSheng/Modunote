use storage::database::{DatabaseResult, test_utils::connect_and_run_migration};

#[tokio::test]
async fn sqlite_database_contract() -> DatabaseResult<()> {
    let database_url = std::env::var("DATABASE_URL")?;
    connect_and_run_migration::<storage_sqlite::SqliteDb>(&database_url).await
}
