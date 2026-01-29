use rstest::fixture;

use storage::database::Database;
use storage_postgres::PostgresDb;

#[fixture]
pub async fn postgres_db() -> PostgresDb {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db = PostgresDb::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    db.run_migration().await.expect("Failed to run migrations");

    db
}
