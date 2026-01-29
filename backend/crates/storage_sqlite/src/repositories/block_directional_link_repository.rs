use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Executor, Sqlite};
use uuid::Uuid;

use crate::helpers::SqliteBlockDirectionalPathHelper;
use domain::blocks::BlockDirectionalLink;
use storage::helpers::{
    block_directional_path_helper::BlockDirectionalPathHelper,
    sqlx_error_kind_helpers::{is_foreign_key_violation, is_unique_violation},
};
use storage::repositories::BlockDirectionalLinkRepository;
use storage::repositories::block_directional_link_repository::{
    BlockDirectionalLinkRepositoryError, BlockDirectionalLinkRepositoryResult as Result,
    CreateBlockDirectionalLinkDto,
};

#[derive(Clone, Debug, Default)]
pub struct SqliteBlockDirectionalLinkRepository {
    path_helper: SqliteBlockDirectionalPathHelper,
}

impl SqliteBlockDirectionalLinkRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl BlockDirectionalLinkRepository<Sqlite> for SqliteBlockDirectionalLinkRepository {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<BlockDirectionalLink>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let link = sqlx::query_as!(
            BlockDirectionalLink,
            r#"
            SELECT 
                id as "id: _" , 
                block_from_id as "block_from_id: _",
                block_to_id as "block_to_id: _",
                created_at as "created_at: _"
            FROM block_directional_links 
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(executor)
        .await?;

        Ok(link)
    }

    async fn create<'e, E>(
        &self,
        input: &CreateBlockDirectionalLinkDto,
        executor: E,
    ) -> Result<BlockDirectionalLink>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let now = Utc::now();

        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        self.ensure_no_cycle(input.block_from_id, input.block_to_id, &mut *tx)
            .await?;

        let link = sqlx::query_as!(
            BlockDirectionalLink,
            r#"
            INSERT INTO block_directional_links
                (id, block_from_id, block_to_id, created_at)
                VALUES ($1, $2, $3, $4)
            RETURNING
                id as "id: _",
                block_from_id as "block_from_id: _",
                block_to_id as "block_to_id: _",
                created_at as "created_at: _"
            "#,
            input.id,
            input.block_from_id,
            input.block_to_id,
            now
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            if is_foreign_key_violation(&e) {
                return BlockDirectionalLinkRepositoryError::BlocksNotFound {
                    from: input.block_from_id,
                    to: input.block_to_id,
                };
            } else if is_unique_violation(&e) {
                return BlockDirectionalLinkRepositoryError::AlreadyExists {
                    from: input.block_from_id,
                    to: input.block_to_id,
                };
            }
            BlockDirectionalLinkRepositoryError::Database(e)
        })?;

        self.path_helper
            .create_paths_for_link(input.block_from_id, input.block_to_id, &mut *tx)
            .await?;

        tx.commit().await?;

        Ok(link)
    }

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        let link = self
            .get_by_id(id, &mut *tx)
            .await?
            .ok_or(BlockDirectionalLinkRepositoryError::NotFoundById { id })?;

        self.path_helper
            .delete_paths_using_link(link.block_from_id, link.block_to_id, &mut *tx)
            .await?;

        sqlx::query!("DELETE FROM block_directional_links WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_by_block_ids<'e, E>(
        &self,
        block_from_id: Uuid,
        block_to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        self.path_helper
            .delete_paths_using_link(block_from_id, block_to_id, &mut *tx)
            .await?;

        let result = sqlx::query!(
            "DELETE FROM block_directional_links WHERE block_from_id = $1 AND block_to_id = $2",
            block_from_id,
            block_to_id
        )
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BlockDirectionalLinkRepositoryError::NotFoundByBlocks {
                from: block_from_id,
                to: block_to_id,
            });
        }

        tx.commit().await?;

        Ok(())
    }
}

impl SqliteBlockDirectionalLinkRepository {
    async fn ensure_no_cycle<'c, E>(
        &self,
        block_from_id: Uuid,
        block_to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        // Check for self-link
        if block_from_id == block_to_id {
            return Err(BlockDirectionalLinkRepositoryError::CycleDetected {
                from: block_from_id,
                to: block_to_id,
            });
        }

        // Check for cycle
        let cycle_detected = self
            .path_helper
            .is_ancestor_descendant(block_to_id, block_from_id, executor)
            .await?;

        if cycle_detected {
            return Err(BlockDirectionalLinkRepositoryError::CycleDetected {
                from: block_from_id,
                to: block_to_id,
            });
        }

        Ok(())
    }
}
