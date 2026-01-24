use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};
use uuid::Uuid;

use crate::helpers::BlockDirectionalPathHelper;
use domain::blocks::BlockDirectionalLink;
use storage::helpers::sqlx_error_kind_helpers::{is_foreign_key_violation, is_unique_violation};
use storage::repositories::BlockDirectionalLinkRepository;
use storage::repositories::block_directional_link_repository::{
    BlockDirectionalLinkRepositoryError, BlockDirectionalLinkRepositoryResult as Result,
    CreateBlockDirectionalLinkDto,
};

#[derive(Clone, Debug)]
pub struct SqliteBlockDirectionalLinkRepository {
    pool: Pool<Sqlite>,
    path_helper: BlockDirectionalPathHelper,
}

impl SqliteBlockDirectionalLinkRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
            path_helper: BlockDirectionalPathHelper,
        }
    }
}

#[async_trait]
impl BlockDirectionalLinkRepository for SqliteBlockDirectionalLinkRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<BlockDirectionalLink>> {
        let link = Self::get_by_id_with_executor(id, &self.pool).await?;

        Ok(link)
    }

    async fn create(&self, input: &CreateBlockDirectionalLinkDto) -> Result<BlockDirectionalLink> {
        let now = Utc::now();

        let mut transaction = self.pool.begin().await?;

        self.ensure_no_cycle(input.block_from_id, input.block_to_id, &mut *transaction)
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
        .fetch_one(&mut *transaction)
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
            .create_paths_for_new_link(input.block_from_id, input.block_to_id, &mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(link)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        let link = Self::get_by_id_with_executor(id, &mut *transaction)
            .await?
            .ok_or(BlockDirectionalLinkRepositoryError::NotFoundById { id })?;

        self.path_helper
            .delete_paths_using_link(link.block_from_id, link.block_to_id, &mut transaction)
            .await?;

        sqlx::query!("DELETE FROM block_directional_links WHERE id = $1", id)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn delete_by_block_ids(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        self.path_helper
            .delete_paths_using_link(block_from_id, block_to_id, &mut transaction)
            .await?;

        let result = sqlx::query!(
            "DELETE FROM block_directional_links WHERE block_from_id = $1 AND block_to_id = $2",
            block_from_id,
            block_to_id
        )
        .execute(&mut *transaction)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BlockDirectionalLinkRepositoryError::NotFoundByBlocks {
                from: block_from_id,
                to: block_to_id,
            });
        }

        transaction.commit().await?;
        Ok(())
    }
}

impl SqliteBlockDirectionalLinkRepository {
    async fn get_by_id_with_executor<'c, E>(
        id: Uuid,
        executor: E,
    ) -> Result<Option<BlockDirectionalLink>>
    where
        E: Executor<'c, Database = Sqlite>,
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
