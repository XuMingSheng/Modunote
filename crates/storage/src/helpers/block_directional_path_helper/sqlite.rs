use std::collections::HashSet;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite, Transaction};
use uuid::Uuid;

use super::{
    error::{BlockDirectionalPathHelperError, BlockDirectionalPathHelperResult as Result},
    traits::BlockDirectionalPathHelperTrait,
};

#[derive(Clone, Debug)]
pub struct SqliteBlockDirectionalPathHelper {
    pool: Pool<Sqlite>,
}

impl SqliteBlockDirectionalPathHelper {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockDirectionalPathHelperTrait for SqliteBlockDirectionalPathHelper {
    // Path existence and relationship queries
    async fn is_ancestor_descendant(&self, ancestor_id: Uuid, descendant_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM block_directional_paths
            WHERE block_ancestor_id = $1 AND block_descendant_id = $2",
            ancestor_id,
            descendant_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    // Single block hierarchy queries
    async fn get_ancestors_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let ancestors = sqlx::query_scalar!(
            r#"SELECT DISTINCT block_ancestor_id as "block_ancestor_id: Uuid"
            FROM block_directional_paths
            WHERE block_descendant_id = $1"#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(ancestors)
    }

    async fn get_descendants_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>> {
        let descendants = sqlx::query_scalar!(
            r#"SELECT DISTINCT block_descendant_id as "block_descendant_id: Uuid"
            FROM block_directional_paths
            WHERE block_ancestor_id = $1"#,
            block_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(descendants)
    }

    // Path enumeration queries
    async fn get_paths_from_id(&self, ancestor_id: Uuid) -> Result<Vec<Vec<Uuid>>> {
        let paths = self
            .get_paths_from_id_with_executor(ancestor_id, &self.pool)
            .await?;
        Ok(paths)
    }

    async fn get_paths_to_id(&self, descendant_id: Uuid) -> Result<Vec<Vec<Uuid>>> {
        let paths = self
            .get_paths_to_id_with_executor(descendant_id, &self.pool)
            .await?;
        Ok(paths)
    }

    async fn get_paths_between_ids(
        &self,
        ancestor_id: Uuid,
        descendant_id: Uuid,
    ) -> Result<Vec<Vec<Uuid>>> {
        let path_strs = sqlx::query_scalar!(
            "SELECT block_path_ids from block_directional_paths
            WHERE block_ancestor_id = $1 AND block_descendant_id = $2",
            ancestor_id,
            descendant_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut paths: Vec<Vec<Uuid>> = Vec::new();

        for path_str in path_strs {
            let path_ids: Vec<Uuid> = serde_json::from_str(&path_str)?;
            paths.push(path_ids);
        }

        Ok(paths)
    }

    async fn get_paths_between_id_sets(
        &self,
        ancestor_ids: HashSet<Uuid>,
        descendant_ids: HashSet<Uuid>,
    ) -> Result<Vec<Vec<Uuid>>> {
        let mut all_paths = Vec::new();

        for ancestor_id in &ancestor_ids {
            for descendant_id in &descendant_ids {
                let paths = self
                    .get_paths_between_ids(*ancestor_id, *descendant_id)
                    .await?;
                all_paths.extend(paths);
            }
        }

        Ok(all_paths)
    }

    // Path maintenance operations (require transaction)
    async fn create_paths_for_new_link(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {
        // Add direct path
        let direct_path = vec![from_id, to_id];
        self.create_path(direct_path, transaction).await?;

        //  Extend all paths ending at from_id
        let from_id_ancestor_paths = self
            .get_paths_to_id_with_executor(from_id, &mut **transaction)
            .await?;
        for path in &from_id_ancestor_paths {
            let mut extended_path = path.clone();
            extended_path.push(to_id);
            self.create_path(extended_path, transaction).await?;
        }

        // Extend all paths starting from to_id
        let to_id_descendant_paths = self
            .get_paths_from_id_with_executor(to_id, &mut **transaction)
            .await?;
        for path in &to_id_descendant_paths {
            let mut extended_path = vec![from_id];
            extended_path.extend(path.iter());
            self.create_path(extended_path, transaction).await?;
        }

        // Connect all ancestors of from_id to all descendants of to_id
        for ancestor_path in &from_id_ancestor_paths {
            for descendant_path in &to_id_descendant_paths {
                let mut combined_path = ancestor_path.clone();
                combined_path.extend(descendant_path.iter());
                self.create_path(combined_path, transaction).await?;
            }
        }

        Ok(())
    }

    async fn delete_path_using_link(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {
        // sqlx stores Uuid as BLOB to sqlite by default (even if it is configured as TEXT type)
        // So we need to manually convert Uuid to string to match the format in paths
        let from_id_str = from_id.to_string();
        let to_id_str = to_id.to_string();

        sqlx::query!(
            "
            DELETE FROM block_directional_paths
            WHERE id IN (
                SELECT bdp.id
                FROM block_directional_paths AS bdp
                WHERE EXISTS (
                    SELECT 1 FROM json_each(bdp.block_path_ids) AS a
                    JOIN json_each(bdp.block_path_ids) AS b ON a.key = b.key - 1
                    WHERE a.value = $1 AND b.value = $2
                )
            )
            ",
            from_id_str,
            to_id_str,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    async fn delete_path_using_block(
        &self,
        block_id: Uuid,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {
        // sqlx stores Uuid as BLOB to sqlite by default (even if it is configured as TEXT type)
        // So we need to manually convert Uuid to string to match the format in paths
        let block_id_str = block_id.to_string();

        sqlx::query!(
            "DELETE FROM block_directional_paths
            WHERE EXISTS (
                SELECT 1 FROM json_each(block_path_ids)
                WHERE value = $1
            )",
            block_id_str
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl SqliteBlockDirectionalPathHelper {
    async fn get_paths_from_id_with_executor<'c, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<Vec<Uuid>>>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        let path_strs = sqlx::query_scalar!(
            "SELECT block_path_ids from block_directional_paths
            WHERE block_ancestor_id = $1",
            block_id,
        )
        .fetch_all(executor)
        .await?;

        let mut paths: Vec<Vec<Uuid>> = Vec::new();

        for path_str in path_strs {
            let path_ids: Vec<Uuid> = serde_json::from_str(&path_str)?;
            paths.push(path_ids);
        }

        Ok(paths)
    }

    async fn get_paths_to_id_with_executor<'c, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<Vec<Uuid>>>
    where
        E: Executor<'c, Database = Sqlite>,
    {
        let path_strs = sqlx::query_scalar!(
            "SELECT block_path_ids from block_directional_paths
            WHERE block_descendant_id = $1",
            block_id,
        )
        .fetch_all(executor)
        .await?;

        let mut paths: Vec<Vec<Uuid>> = Vec::new();

        for path_str in path_strs {
            let path_ids: Vec<Uuid> = serde_json::from_str(&path_str)?;
            paths.push(path_ids);
        }

        Ok(paths)
    }

    async fn create_path(
        &self,
        path_ids: Vec<Uuid>,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()> {
        self.check_empty_path(&path_ids)?;
        self.check_self_path(&path_ids)?;

        let id = Uuid::new_v4();
        let ancestor_id = path_ids.first().unwrap();
        let descendant_id = path_ids.last().unwrap();

        let path_json = serde_json::to_string(&path_ids)?;
        let path_len = path_ids.len() as i32;
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO block_directional_paths
            (id, block_ancestor_id, block_descendant_id, block_path_ids, path_length, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            ancestor_id,
            descendant_id,
            path_json,
            path_len,
            now,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    fn check_empty_path(&self, path_ids: &[Uuid]) -> Result<()> {
        if path_ids.is_empty() {
            return Err(BlockDirectionalPathHelperError::EmptyPath);
        }
        Ok(())
    }

    fn check_self_path(&self, path_ids: &[Uuid]) -> Result<()> {
        if path_ids.len() == 1 {
            return Err(BlockDirectionalPathHelperError::SelfPath { id: path_ids[0] });
        }
        Ok(())
    }
}
