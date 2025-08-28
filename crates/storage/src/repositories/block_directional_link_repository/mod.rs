mod error;
mod traits;

#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{BlockDirectionalLinkError, BlockDirectionalLinkResult};
pub use traits::BlockDirectionalLinkRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteBlockDirectionalLinkRepository as BlockDirectionalLinkRepository;
