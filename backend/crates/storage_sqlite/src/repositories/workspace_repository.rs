use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use domain::workspaces::workspace::{OpenedBlock, Workspace};
use storage::helpers::sqlx_error_kind_helpers::is_foreign_key_violation;
use storage::repositories::WorkspaceRepository;
use storage::repositories::workspace_repository::{
    WorkspaceRepositoryError, WorkspaceRepostoryResult as Result,
};

#[derive(Clone, Debug)]
pub struct SqliteWorkspaceRepository {
    pool: Pool<Sqlite>,
}

impl SqliteWorkspaceRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl WorkspaceRepository for SqliteWorkspaceRepository {
    async fn get(&self) -> Result<Workspace> {
        let opened_blocks = sqlx::query_as!(
            OpenedBlock,
            r#"
            SELECT
                block_id as "block_id: _",
                opened_at as "opened_at: _",
                tab_order as "tab_order: _"
            FROM block_opens
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let workspace = Workspace { opened_blocks };

        Ok(workspace)
    }

    async fn save(&self, block_workspace: &Workspace) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!("DELETE FROM block_opens")
            .execute(&mut *transaction)
            .await?;

        for opened_block in &block_workspace.opened_blocks {
            sqlx::query!(
                r#"
                INSERT INTO block_opens (block_id, opened_at, tab_order)
                VALUES ($1, $2, $3)
                "#,
                opened_block.block_id,
                opened_block.opened_at,
                opened_block.tab_order
            )
            .execute(&mut *transaction)
            .await
            .map_err(|e| {
                if is_foreign_key_violation(&e) {
                    return WorkspaceRepositoryError::SomeBlocksNotFound;
                }
                WorkspaceRepositoryError::Database(e)
            })?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
