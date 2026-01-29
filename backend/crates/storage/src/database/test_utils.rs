use super::{Database, DatabaseResult};

#[cfg(feature = "test-utils")]
pub async fn connect_and_run_migration<D: Database>(database_url: &str) -> DatabaseResult<()> {
    let db = D::connect(database_url).await?;
    db.run_migration().await?;
    Ok(())
}
