use async_trait::async_trait;
use sqlx::{Acquire, Database, Executor};

use super::error::WorkspaceRepostoryResult as Result;
use domain::workspaces::Workspace;

#[async_trait]
pub trait WorkspaceRepository<DB: Database>: Send + Sync {
    async fn get<'e, E>(&self, executor: E) -> Result<Workspace>
    where
        E: Executor<'e, Database = DB>;

    async fn save<'e, E>(&self, workspace: &Workspace, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;
}
