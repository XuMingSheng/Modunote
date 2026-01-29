use storage::database::{DatabaseResult, test_utils::connect_and_run_migration};

#[tokio::test]
async fn postgres_database_contract() -> DatabaseResult<()> {
    let database_url = match std::env::var("POSTGRES_TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Ok(()),
    };

    connect_and_run_migration::<storage_postgres::PostgresDb>(&database_url).await
}
