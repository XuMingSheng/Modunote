use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

use storage::query_services::BlockLinkQueryService;
use storage::query_services::block_link_query_service::{
    AllLinkedBlocksDto, BlockLinkQueryServiceResult as Result, LinkedBlockDto,
};

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
enum LinkType {
    Parent,
    Child,
    Related,
}

struct LinkedBlockModel {
    link_type: LinkType,
    link_id: Uuid,
    block_id: Uuid,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<&LinkedBlockModel> for LinkedBlockDto {
    fn from(model: &LinkedBlockModel) -> Self {
        Self {
            link_id: model.link_id,
            block_id: model.block_id,
            title: model.title.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PostgresBlockLinkQueryService;

impl PostgresBlockLinkQueryService {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl BlockLinkQueryService<Postgres> for PostgresBlockLinkQueryService {
    async fn get_linked_blocks<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<AllLinkedBlocksDto>
    where
        E: Executor<'e, Database = Postgres>,
    {
        // SQLx primarily looks at the first SELECT for the schema definition.
        let linked_blocks = sqlx::query_as!(
            LinkedBlockModel,
            r#"
            SELECT
                'parent' as "link_type!: LinkType ",
                bdl.id as "link_id!",
                b.id as "block_id!",
                b.title as "title!",
                b.created_at as "created_at!",
                b.updated_at as "updated_at!"
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_from_id
            WHERE bdl.block_to_id = $1
            UNION ALL
            SELECT
                'child',
                bdl.id,
                b.id,
                b.title,
                b.created_at,
                b.updated_at
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_to_id
            WHERE bdl.block_from_id = $1
            UNION ALL
            SELECT
                'related',
                brl.id,
                b.id,
                b.title,
                b.created_at,
                b.updated_at
            FROM block_related_links brl
            JOIN blocks b ON (
                (brl.block_a_id = $1 AND b.id = brl.block_b_id) OR 
                (brl.block_b_id = $1 AND b.id = brl.block_a_id)
            )
            WHERE brl.block_a_id = $1 OR brl.block_b_id = $1
            "#,
            block_id
        )
        .fetch_all(executor)
        .await?;

        let mut parent_blocks = Vec::new();
        let mut child_blocks = Vec::new();
        let mut related_blocks = Vec::new();

        for linked_block in &linked_blocks {
            let dto: LinkedBlockDto = linked_block.into();

            match linked_block.link_type {
                LinkType::Parent => parent_blocks.push(dto),
                LinkType::Child => child_blocks.push(dto),
                LinkType::Related => related_blocks.push(dto),
            }
        }

        Ok(AllLinkedBlocksDto {
            parent_blocks,
            child_blocks,
            related_blocks,
        })
    }

    async fn get_parent_blocks<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let parent_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                bdl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at,
                b.updated_at
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_from_id
            WHERE bdl.block_to_id = $1
            "#,
            block_id
        )
        .fetch_all(executor)
        .await?;

        Ok(parent_blocks)
    }

    async fn get_child_blocks<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let child_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                bdl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at,
                b.updated_at
            FROM block_directional_links bdl
            JOIN blocks b ON b.id = bdl.block_to_id
            WHERE bdl.block_from_id = $1
            "#,
            block_id
        )
        .fetch_all(executor)
        .await?;

        Ok(child_blocks)
    }

    async fn get_related_blocks<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let related_blocks = sqlx::query_as!(
            LinkedBlockDto,
            r#"
            SELECT
                brl.id as "link_id: _",
                b.id as "block_id: _",
                b.title,
                b.created_at,
                b.updated_at
            FROM block_related_links brl
            JOIN blocks b ON (
                (brl.block_a_id = $1 AND b.id = brl.block_b_id) OR 
                (brl.block_b_id = $1 AND b.id = brl.block_a_id)
            )
            WHERE brl.block_a_id = $1 OR brl.block_b_id = $1
            "#,
            block_id
        )
        .fetch_all(executor)
        .await?;

        Ok(related_blocks)
    }
}
