use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Executor, Postgres};
use uuid::Uuid;

use domain::blocks::BlockRelatedLink;
use storage::helpers::sqlx_error_kind_helpers::{is_foreign_key_violation, is_unique_violation};
use storage::repositories::BlockRelatedLinkRepository;
use storage::repositories::block_related_link_repository::{
    BlockRelatedLinkError, BlockRelatedLinkResult as Result, CreateBlockRelatedLinkDto,
};

#[derive(Clone, Debug, Default)]
pub struct PostgresBlockRelatedLinkRepository {}

impl PostgresBlockRelatedLinkRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl BlockRelatedLinkRepository<Postgres> for PostgresBlockRelatedLinkRepository {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<BlockRelatedLink>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let link = sqlx::query_as!(
            BlockRelatedLink,
            r#"
            SELECT
                id,
                block_a_id,
                block_b_id,
                created_at
            FROM block_related_links
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
        input: &CreateBlockRelatedLinkDto,
        executor: E,
    ) -> Result<BlockRelatedLink>
    where
        E: Executor<'e, Database = Postgres> + Acquire<'e, Database = Postgres>,
    {
        let now = Utc::now();

        Self::ensure_no_selflink(input.block_a_id, input.block_b_id)?;

        let (block_a_id, block_b_id) = Self::ordered_ids(input.block_a_id, input.block_b_id);

        let link = sqlx::query_as!(
            BlockRelatedLink,
            r#"
            INSERT INTO block_related_links
                (id, block_a_id, block_b_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                block_a_id,
                block_b_id,
                created_at
            "#,
            input.id,
            block_a_id,
            block_b_id,
            now
        )
        .fetch_one(executor)
        .await
        .map_err(|e| {
            if is_foreign_key_violation(&e) {
                return BlockRelatedLinkError::BlocksNotFound {
                    a: block_a_id,
                    b: block_b_id,
                };
            } else if is_unique_violation(&e) {
                return BlockRelatedLinkError::AlreadyExists {
                    a: block_a_id,
                    b: block_b_id,
                };
            }
            BlockRelatedLinkError::Database(e)
        })?;

        Ok(link)
    }

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Postgres> + Acquire<'e, Database = Postgres>,
    {
        let result = sqlx::query!("DELETE FROM block_related_links WHERE id = $1", id)
            .execute(executor)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BlockRelatedLinkError::NotFoundById { id });
        }

        Ok(())
    }

    async fn delete_by_block_ids<'e, E>(
        &self,
        block_a_id: Uuid,
        block_b_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = Postgres> + Acquire<'e, Database = Postgres>,
    {
        let (block_a_id, block_b_id) = Self::ordered_ids(block_a_id, block_b_id);

        let result = sqlx::query!(
            "DELETE FROM block_related_links WHERE block_a_id = $1 AND block_b_id = $2",
            block_a_id,
            block_b_id
        )
        .execute(executor)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BlockRelatedLinkError::NotFoundByBlocks {
                a: block_a_id,
                b: block_b_id,
            });
        }

        Ok(())
    }
}

impl PostgresBlockRelatedLinkRepository {
    fn ensure_no_selflink(block_a_id: Uuid, block_b_id: Uuid) -> Result<()> {
        if block_a_id == block_b_id {
            return Err(BlockRelatedLinkError::SelfLink { id: block_a_id });
        }

        Ok(())
    }

    fn ordered_ids(block_a_id: Uuid, block_b_id: Uuid) -> (Uuid, Uuid) {
        if block_a_id > block_b_id {
            (block_b_id, block_a_id)
        } else {
            (block_a_id, block_b_id)
        }
    }
}
