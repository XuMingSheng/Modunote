mod error;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{DatabaseError, DatabaseResult};

#[cfg(feature = "postgres")]
pub use postgres::PostgresDb as Database;
#[cfg(feature = "sqlite")]
pub use sqlite::SqliteDb as Database;
