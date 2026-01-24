use async_trait::async_trait;

use super::error::WorkspaceRepostoryResult as Result;
use domain::workspaces::Workspace;

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn get(&self) -> Result<Workspace>;
    async fn save(&self, workspace: &Workspace) -> Result<()>;
}
