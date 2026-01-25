use std::collections::HashSet;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Executor, Sqlite};
use uuid::Uuid;

use storage::helpers::block_directional_path_helper::{
    BlockDirectionalPathHelper, BlockDirectionalPathHelperError,
    BlockDirectionalPathHelperResult as Result,
};
use storage::helpers::sqlx_error_kind_helpers::{is_foreign_key_violation, is_unique_violation};

#[derive(Clone, Debug, Default)]
pub struct SqliteBlockDirectionalPathHelper;

#[async_trait]
impl BlockDirectionalPathHelper<Sqlite> for SqliteBlockDirectionalPathHelper {
    async fn is_ancestor_descendant<'e, E>(
        &self,
        ancestor_id: Uuid,
        descendant_id: Uuid,
        executor: E,
    ) -> Result<bool>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let count = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM block_directional_paths
            WHERE block_ancestor_id = $1 AND block_descendant_id = $2)",
            ancestor_id,
            descendant_id,
        )
        .fetch_one(executor)
        .await?;

        Ok(count > 0)
    }

    async fn create_paths_for_link<'e, E>(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        // Add direct path
        let direct_path = vec![from_id, to_id];
        self.create_path(direct_path, &mut *tx).await?;

        //  Extend all paths ending at from_id
        let from_id_ancestor_paths = self.get_paths_to_block(from_id, &mut *tx).await?;
        for path in &from_id_ancestor_paths {
            let mut extended_path = path.clone();
            extended_path.push(to_id);
            self.create_path(extended_path, &mut *tx).await?;
        }

        // Extend all paths starting from to_id
        let to_id_descendant_paths = self.get_paths_from_block(to_id, &mut *tx).await?;
        for path in &to_id_descendant_paths {
            let mut extended_path = vec![from_id];
            extended_path.extend(path.iter());
            self.create_path(extended_path, &mut *tx).await?;
        }

        // Connect all ancestors of from_id to all descendants of to_id
        for ancestor_path in &from_id_ancestor_paths {
            for descendant_path in &to_id_descendant_paths {
                let mut combined_path = ancestor_path.clone();
                combined_path.extend(descendant_path.iter());
                self.create_path(combined_path, &mut *tx).await?;
            }
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete_paths_using_link<'e, E>(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite>,
    {
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
        .execute(executor)
        .await?;

        Ok(())
    }

    async fn delete_paths_using_block<'e, E>(&self, block_id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite>,
    {
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
        .execute(executor)
        .await?;

        Ok(())
    }
}

impl SqliteBlockDirectionalPathHelper {
    async fn get_paths_from_block<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<Vec<Vec<Uuid>>>
    where
        E: Executor<'e, Database = Sqlite>,
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

    async fn get_paths_to_block<'e, E>(&self, block_id: Uuid, executor: E) -> Result<Vec<Vec<Uuid>>>
    where
        E: Executor<'e, Database = Sqlite>,
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

    async fn create_path<'e, E>(&self, path_ids: Vec<Uuid>, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        self.ensure_nonempty_path(&path_ids)?;
        self.ensure_noncyclic_path(&path_ids)?;

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
        .execute(executor)
        .await
        .map_err(|e| {
            if is_foreign_key_violation(&e) {
                return BlockDirectionalPathHelperError::PathBlocksNotFound {
                    from: *ancestor_id,
                    to: *descendant_id,
                };
            } else if is_unique_violation(&e) {
                return BlockDirectionalPathHelperError::AlreadyExists {
                    from: *ancestor_id,
                    to: *descendant_id,
                };
            }
            BlockDirectionalPathHelperError::Database(e)
        })?;

        Ok(())
    }

    fn ensure_nonempty_path(&self, path_ids: &[Uuid]) -> Result<()> {
        if path_ids.is_empty() {
            return Err(BlockDirectionalPathHelperError::EmptyPathCreation);
        }
        Ok(())
    }

    fn ensure_noncyclic_path(&self, path_ids: &[Uuid]) -> Result<()> {
        let unique_ids: HashSet<_> = path_ids.iter().collect();
        if path_ids.len() != unique_ids.len() {
            return Err(BlockDirectionalPathHelperError::CyclicPathCreation);
        }
        Ok(())
    }
}
