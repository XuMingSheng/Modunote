use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Acquire, Executor, Postgres};
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
pub struct PostgresWorkspaceRepository {}

impl PostgresWorkspaceRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl WorkspaceRepository<Postgres> for PostgresWorkspaceRepository {
    async fn get<'e, E>(&self, executor: E) -> Result<Workspace>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let opened_blocks: Vec<OpenedBlock> = sqlx::query_as!(
            OpenedBlockModel,
            r#"
            SELECT
                block_id,
                opened_at,
                tab_index
            FROM block_opens
            "#
        )
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(OpenedBlock::from)
        .collect();

        Ok(Workspace { opened_blocks })
    }

    async fn save<'e, E>(&self, workspace: &Workspace, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Postgres> + Acquire<'e, Database = Postgres>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        sqlx::query!("DELETE FROM block_opens")
            .execute(&mut *tx)
            .await?;

        for opened_block in &workspace.opened_blocks {
            sqlx::query!(
                r#"
                INSERT INTO block_opens (block_id, opened_at, tab_index)
                VALUES ($1, $2, $3)
                "#,
                opened_block.block_id,
                opened_block.opened_at,
                opened_block.tab_index as i32
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
