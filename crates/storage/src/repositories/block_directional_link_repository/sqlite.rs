use std::collections::HashSet;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};
use uuid::Uuid;

use super::{
    error::{BlockDirectionalLinkError, BlockDirectionalLinkResult as Result},
    traits::BlockDirectionalLinkRepositoryTrait,
};
use crate::{
    BlockDirectionalLink, BlockDirectionalLinkCreate, BlockSummary,
    helpers::{BlockDirectionalPathHelper, BlockDirectionalPathHelperTrait},
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
            path_helper: BlockDirectionalPathHelper::new(pool),
        }
    }
}

#[async_trait]
impl BlockDirectionalLinkRepositoryTrait for SqliteBlockDirectionalLinkRepository {
    async fn get_children_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let children = sqlx::query_scalar!(
            r#"
            SELECT block_to_id as "block_to_id: Uuid" 
            FROM block_directional_links WHERE block_from_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(children)
    }

    async fn get_parents_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let parents = sqlx::query_scalar!(
            r#"
            SELECT block_from_id as "block_from_id: Uuid"
            FROM block_directional_links WHERE block_to_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(parents)
    }

    async fn get_children_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>> {
        let children = sqlx::query_as!(
            BlockSummary,
            r#"
            SELECT
                b.id as "id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _",
                bp.pinned_at as "pinned_at: _"
            FROM block_directional_links bdl
            JOIN blocks b ON bdl.block_to_id = b.id
            LEFT JOIN block_opens bo ON b.id = bo.block_id 
            LEFT JOIN block_pins bp ON b.id = bp.block_id
            WHERE bdl.block_from_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(children)
    }

    async fn get_parents_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>> {
        let parents = sqlx::query_as!(
            BlockSummary,
            r#"
            SELECT
                b.id as "id: _",
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _",
                bp.pinned_at as "pinned_at: _"
            FROM block_directional_links bdl
            JOIN blocks b ON bdl.block_from_id = b.id
            LEFT JOIN block_opens bo ON b.id = bo.block_id 
            LEFT JOIN block_pins bp ON b.id = bp.block_id
            WHERE bdl.block_to_id = $1
            "#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(parents)
    }

    async fn exists(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM block_directional_links 
            WHERE block_from_id = $1 AND block_to_id = $2",
            block_from_id,
            block_to_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn create(
        &self,
        link_create: BlockDirectionalLinkCreate,
    ) -> Result<BlockDirectionalLink> {
        let now = Utc::now();

        self.check_no_cycle_form(link_create.block_from_id, link_create.block_to_id)
            .await?;

        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            "
            INSERT INTO block_directional_links
            (id, block_from_id, block_to_id, created_at)
            VALUES ($1, $2, $3, $4)",
            link_create.id,
            link_create.block_from_id,
            link_create.block_to_id,
            now
        )
        .execute(&mut *transaction)
        .await?;

        self.path_helper
            .create_paths_for_new_link(
                link_create.block_from_id,
                link_create.block_to_id,
                &mut transaction,
            )
            .await?;

        transaction.commit().await?;

        self.get_by_id_with_executor(link_create.id, &self.pool)
            .await?
            .ok_or(BlockDirectionalLinkError::NotFound {
                from: link_create.block_from_id,
                to: link_create.block_to_id,
            })
    }

    async fn delete(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        self.path_helper
            .delete_path_using_link(block_from_id, block_to_id, &mut transaction)
            .await?;

        let result = sqlx::query!(
            "DELETE FROM block_directional_links 
            WHERE block_from_id = $1 and block_to_id = $2",
            block_from_id,
            block_to_id,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        if result.rows_affected() == 0 {
            return Err(BlockDirectionalLinkError::NotFound {
                from: block_from_id,
                to: block_to_id,
            });
        }

        Ok(())
    }

    async fn is_ancestor_descendant(&self, ancestor_id: Uuid, descendant_id: Uuid) -> Result<bool> {
        Ok(self
            .path_helper
            .is_ancestor_descendant(ancestor_id, descendant_id)
            .await?)
    }

    async fn get_descendants_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let descendants = self.path_helper.get_descendants_ids(block_id).await?;
        Ok(descendants)
    }

    async fn get_ancestors_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let ancestors = self.path_helper.get_ancestors_ids(block_id).await?;
        Ok(ancestors)
    }

    async fn get_root_ids(&self) -> Result<Vec<Uuid>> {
        let roots = sqlx::query_scalar!(
            r#"SELECT DISTINCT b.id as "id: Uuid"
            FROM blocks b
            LEFT JOIN block_directional_links bdl ON b.id = bdl.block_to_id
            WHERE bdl.block_to_id IS NULL"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(roots)
    }

    async fn get_leaf_ids(&self) -> Result<Vec<Uuid>> {
        let leaves = sqlx::query_scalar!(
            r#"SELECT DISTINCT b.id as "id: Uuid"
            FROM blocks b
            LEFT JOIN block_directional_links bdl ON b.id = bdl.block_from_id
            WHERE bdl.block_from_id IS NULL"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(leaves)
    }

    async fn get_paths_from_id(&self, ancestor_id: Uuid) -> Result<Vec<Vec<Uuid>>> {
        let paths = self.path_helper.get_paths_from_id(ancestor_id).await?;
        Ok(paths)
    }

    async fn get_paths_to_id(&self, descendant_id: Uuid) -> Result<Vec<Vec<Uuid>>> {
        let paths = self.path_helper.get_paths_to_id(descendant_id).await?;
        Ok(paths)
    }

    async fn get_paths_between_ids(
        &self,
        ancestor_id: Uuid,
        descendant_id: Uuid,
    ) -> Result<Vec<Vec<Uuid>>> {
        let paths = self
            .path_helper
            .get_paths_between_ids(ancestor_id, descendant_id)
            .await?;
        Ok(paths)
    }

    async fn get_paths_between_id_sets(
        &self,
        ancestor_ids: HashSet<Uuid>,
        descendant_ids: HashSet<Uuid>,
    ) -> Result<Vec<Vec<Uuid>>> {
        let paths = self
            .path_helper
            .get_paths_between_id_sets(ancestor_ids, descendant_ids)
            .await?;
        Ok(paths)
    }
}

impl SqliteBlockDirectionalLinkRepository {
    async fn get_by_id_with_executor<'c, E>(
        &self,
        link_id: Uuid,
        executor: E,
    ) -> Result<Option<BlockDirectionalLink>>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        let link = sqlx::query_as!(
            BlockDirectionalLink,
            r#"
            SELECT
                id as "id: _", 
                block_from_id as "block_from_id: _", 
                block_to_id as "block_to_id: _", 
                created_at as "created_at: _"
            FROM block_directional_links
            WHERE id = $1
            "#,
            link_id,
        )
        .fetch_optional(executor)
        .await?;

        Ok(link)
    }

    async fn check_no_cycle_form(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<()> {
        // Check for self-link
        if block_from_id == block_to_id {
            return Err(BlockDirectionalLinkError::SelfLink { id: block_from_id });
        }

        // Check for cycle
        let form_cycle = self
            .path_helper
            .is_ancestor_descendant(block_to_id, block_from_id)
            .await?;

        if form_cycle {
            return Err(BlockDirectionalLinkError::CycleDetected {
                from: block_from_id,
                to: block_to_id,
            });
        }

        Ok(())
    }
}
