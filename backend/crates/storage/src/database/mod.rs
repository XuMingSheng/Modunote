mod error;
mod traits;

#[cfg(feature = "test-utils")]
pub mod test_utils;

pub use error::{DatabaseError, DatabaseResult};
pub use traits::Database;
