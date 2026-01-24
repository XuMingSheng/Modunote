pub fn is_unique_violation(error: &sqlx::Error) -> bool {
    if let Some(db_error) = error.as_database_error() {
        return db_error.is_unique_violation();
    }
    false
}

pub fn is_foreign_key_violation(error: &sqlx::Error) -> bool {
    if let Some(db_error) = error.as_database_error() {
        return db_error.is_foreign_key_violation();
    }
    false
}
