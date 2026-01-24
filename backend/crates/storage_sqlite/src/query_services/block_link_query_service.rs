use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use storage::query_services::BlockLinkQueryService;
use storage::query_services::block_link_query_service::{
    AllLinkedBlocksDto, BlockLinkQueryServiceResult as Result, LinkedBlockDto,
};

#[derive(Clone, Debug)]
pub struct SqliteBlockLinkQueryService {
    pool: Pool<Sqlite>,
}

impl SqliteBlockLinkQueryService {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockLinkQueryService for SqliteBlockLinkQueryService {
    async fn get_linked_blocks(&self, block_id: Uuid) -> Result<AllLinkedBlocksDto> {
        let parent_blocks = self.get_related_blocks(block_id).await?;
        let child_blocks = self.get_child_blocks(block_id).await?;
        let related_blocks = self.get_related_blocks(block_id).await?;

        let linked_blocks = AllLinkedBlocksDto {
            parent_blocks,
            child_blocks,
            related_blocks,
        };

        Ok(linked_blocks)
    }

    async fn get_parent_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>> {
        let parent_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                bdl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _"
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_from_id
            WHERE bdl.block_to_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(parent_blocks)
    }

    async fn get_child_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>> {
        let child_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                bdl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _"
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_to_id
            WHERE bdl.block_from_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(child_blocks)
    }

    async fn get_related_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>> {
        let related_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                brl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _"
            FROM block_related_links brl
            JOIN blocks b ON (
                CASE
                    WHEN brl.block_a_id = $1 THEN b.id = brl.block_b_id
                    ELSE  b.id = brl.block_a_id
                END
            )
            WHERE brl.block_a_id = $1 OR brl.block_b_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(related_blocks)
    }
}
