pub mod error;
pub mod traits;

#[cfg(feature = "sqlite")]
pub mod sqlite;

pub use error::{BlockPinError, BlockPinResult};
pub use traits::BlockPinRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteBlockPinRepository as BlockPinRepository;