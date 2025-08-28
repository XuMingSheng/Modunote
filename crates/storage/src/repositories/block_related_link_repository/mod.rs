mod error;
mod traits;

#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{BlockRelatedLinkError, BlockRelatedLinkResult};
pub use traits::BlockRelatedLinkRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteBlockRelatedLinkRepository as BlockRelatedLinkRepository;
