use async_trait::async_trait;
use sqlx::{Database as SqlxDb, Pool};

use super::DatabaseResult as Result;

/// A trait for database connections that abstracts different database backends
/// and provides lifecycle management for database connections and migrations.
#[async_trait]
pub trait Database {
    type Provider: SqlxDb;
    /// Returns a reference to the underlying SQLx connection pool.
    fn pool(&self) -> &Pool<Self::Provider>;

    /// Establishes a connection to the database using the provided URL.
    ///
    /// # Arguments
    ///
    /// * `database_url` - The connection string for the database (e.g., "sqlite://path/to/db.sqlite")
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the initialized database instance or an error
    /// if the connection fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let db = SqliteDatabase::connect("sqlite://./app.db").await?;
    /// ```
    async fn connect(database_url: &str) -> Result<Self>
    where
        Self: Sized;

    /// Runs database migrations to ensure the schema is up to date.
    ///
    /// This method should be called after connecting to apply any pending
    /// schema changes or initial table creation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the migration process.
    async fn run_migration(&self) -> Result<()>;
}
