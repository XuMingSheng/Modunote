mod error;
mod traits;

#[cfg(feature = "sqlite")]
mod sqlite;

pub use error::{CanvasError, CanvasResult};
pub use traits::CanvasRepositoryTrait;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteCanvasRepository as CanvasRepository;
