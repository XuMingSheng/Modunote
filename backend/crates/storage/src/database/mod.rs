mod error;
pub mod test_utils;
mod traits;

pub use error::{DatabaseError, DatabaseResult};
pub use traits::Database;
