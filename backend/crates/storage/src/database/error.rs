#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    Connection(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Enviroment varable error {0}")]
    Var(#[from] std::env::VarError),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
