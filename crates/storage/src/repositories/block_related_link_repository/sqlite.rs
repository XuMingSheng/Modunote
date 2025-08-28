use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};
use uuid::Uuid;

use super::{
    error::{BlockRelatedLinkError, BlockRelatedLinkResult as Result},
    traits::BlockRelatedLinkRepositoryTrait,
};
use crate::{BlockRelatedLink, BlockRelatedLinkCreate, BlockSummary};

#[derive(Clone, Debug)]
pub struct SqliteBlockRelatedLinkRepository {
    pool: Pool<Sqlite>,
}

impl SqliteBlockRelatedLinkRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockRelatedLinkRepositoryTrait for SqliteBlockRelatedLinkRepository {
    async fn exists(&self, block_a_id: Uuid, block_b_id: Uuid) -> Result<bool> {
        let (ordered_a, ordered_b) = Self::ordered_pair(block_a_id, block_b_id);

        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM block_related_links
            WHERE block_a_id = $1 AND block_b_id = $2",
            ordered_a,
            ordered_b,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn get_related_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let related = sqlx::query_scalar!(
            r#"
            SELECT
                CASE
                    WHEN block_a_id = $1 THEN block_b_id
                    ELSE block_a_id
                END as "related_id!: Uuid"
            FROM block_related_links
            WHERE block_a_id = $1 OR block_b_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(related)
    }

    async fn get_related_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>> {
        let related = sqlx::query_as!(
            BlockSummary,
            r#"
            SELECT
                b.id as "id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _",
                bp.pinned_at as "pinned_at: _"
            FROM block_related_links brl
            JOIN blocks b ON (
                CASE
                    WHEN brl.block_a_id = $1 THEN brl.block_b_id = b.id
                    ELSE brl.block_a_id = b.id
                END
            )
            LEFT JOIN block_opens bo ON b.id = bo.block_id
            LEFT JOIN block_pins bp ON b.id = bp.block_id
            WHERE brl.block_a_id = $1 OR brl.block_b_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(related)
    }

    async fn create(&self, link_create: BlockRelatedLinkCreate) -> Result<BlockRelatedLink> {
        Self::check_block_ids_is_distinct(link_create.block_a_id, link_create.block_b_id)?;

        let now = Utc::now();
        let (ordered_a, ordered_b) =
            Self::ordered_pair(link_create.block_a_id, link_create.block_b_id);

        sqlx::query!(
            "INSERT INTO block_related_links
            (id, block_a_id, block_b_id, created_at)
            VALUES ($1, $2, $3, $4)",
            link_create.id,
            ordered_a,
            ordered_b,
            now
        )
        .execute(&self.pool)
        .await?;

        self.get_by_id_with_executor(link_create.id, &self.pool)
            .await?
            .ok_or(BlockRelatedLinkError::NotFound {
                a: link_create.block_a_id,
                b: link_create.block_b_id,
            })
    }

    async fn delete(&self, block_a_id: Uuid, block_b_id: Uuid) -> Result<()> {
        let (ordered_a, ordered_b) = Self::ordered_pair(block_a_id, block_b_id);

        let result = sqlx::query!(
            "DELETE FROM block_related_links
            WHERE block_a_id = $1 AND block_b_id = $2",
            ordered_a,
            ordered_b,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(BlockRelatedLinkError::NotFound {
                a: block_a_id,
                b: block_b_id,
            });
        }

        Ok(())
    }
}

impl SqliteBlockRelatedLinkRepository {
    fn ordered_pair(block_a_id: Uuid, block_b_id: Uuid) -> (Uuid, Uuid) {
        if block_a_id < block_b_id {
            (block_a_id, block_b_id)
        } else {
            (block_b_id, block_a_id)
        }
    }

    async fn get_by_id_with_executor<'c, E>(
        &self,
        link_id: Uuid,
        executor: E,
    ) -> Result<Option<BlockRelatedLink>>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        let link = sqlx::query_as!(
            BlockRelatedLink,
            r#"
            SELECT
                id as "id: _",
                block_a_id as "block_a_id: _",
                block_b_id as "block_b_id: _",
                created_at as "created_at: _"
            FROM block_related_links
            WHERE id = $1
            "#,
            link_id,
        )
        .fetch_optional(executor)
        .await?;

        Ok(link)
    }

    fn check_block_ids_is_distinct(block_a_id: Uuid, block_b_id: Uuid) -> Result<()> {
        if block_a_id == block_b_id {
            Err(BlockRelatedLinkError::SelfLink { id: block_a_id })
        } else {
            Ok(())
        }
    }
}