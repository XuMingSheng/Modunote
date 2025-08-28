mod error;
mod traits;

#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{BlockError, BlockResult};
pub use traits::BlockRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteBlockRepository as BlockRepository;
