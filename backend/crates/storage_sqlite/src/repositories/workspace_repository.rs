use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Acquire, Executor, Sqlite};
use uuid::Uuid;

use domain::workspaces::{OpenedBlock, Workspace};
use storage::helpers::sqlx_error_kind_helpers::is_foreign_key_violation;
use storage::repositories::WorkspaceRepository;
use storage::repositories::workspace_repository::{
    WorkspaceRepositoryError, WorkspaceRepostoryResult as Result,
};

struct OpenedBlockModel {
    pub block_id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub tab_index: i32,
}

impl From<OpenedBlockModel> for OpenedBlock {
    fn from(model: OpenedBlockModel) -> Self {
        Self {
            block_id: model.block_id,
            opened_at: model.opened_at,
            tab_index: model.tab_index as usize,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SqliteWorkspaceRepository {}

impl SqliteWorkspaceRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl WorkspaceRepository<Sqlite> for SqliteWorkspaceRepository {
    async fn get<'e, E>(&self, executor: E) -> Result<Workspace>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let opened_blocks = sqlx::query_as!(
            OpenedBlockModel,
            r#"
            SELECT
                block_id as "block_id: _",
                opened_at as "opened_at: _",
                tab_index as "tab_index: _"
            FROM block_opens
            "#
        )
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(OpenedBlock::from)
        .collect();

        let workspace = Workspace { opened_blocks };

        Ok(workspace)
    }

    async fn save<'e, E>(&self, workspace: &Workspace, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        sqlx::query!("DELETE FROM block_opens")
            .execute(&mut *tx)
            .await?;

        for opened_block in &workspace.opened_blocks {
            let tab_index = opened_block.tab_index as i32;
            sqlx::query!(
                r#"
                INSERT INTO block_opens (block_id, opened_at, tab_index)
                VALUES ($1, $2, $3)
                "#,
                opened_block.block_id,
                opened_block.opened_at,
                tab_index
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                if is_foreign_key_violation(&e) {
                    return WorkspaceRepositoryError::SomeBlocksNotFound;
                }
                WorkspaceRepositoryError::Database(e)
            })?;
        }

        tx.commit().await?;

        Ok(())
    }
}
