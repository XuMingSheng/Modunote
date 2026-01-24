mod error;
mod traits;

#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{CanvasBlockError, CanvasBlockResult};
pub use traits::CanvasBlockRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteCanvasBlockRepository as CanvasBlockRepository;
