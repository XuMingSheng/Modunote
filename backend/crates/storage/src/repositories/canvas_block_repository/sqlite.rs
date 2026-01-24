use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::error::{CanvasBlockError, CanvasBlockResult as Result};
use super::traits::CanvasBlockRepositoryTrait;
use crate::entities::{BlockInCanvas, CanvasBlock, CanvasBlockCreate, CanvasBlockUpdate};

#[derive(Clone, Debug)]
pub struct SqliteCanvasBlockRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCanvasBlockRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl CanvasBlockRepositoryTrait for SqliteCanvasBlockRepository {
    async fn get_by_canvas_id(&self, canvas_id: Uuid) -> Result<Vec<CanvasBlock>> {
        self.check_canvas_exists(canvas_id).await?;

        let canvas_blocks = sqlx::query_as!(
            CanvasBlock,
            r#"
            SELECT
                id as "id: _",
                canvas_id as "canvas_id: _",
                block_id as "block_id: _",
                grid_x as "grid_x: _",
                grid_y as "grid_y: _",
                grid_width as "grid_width: _",
                grid_height as "grid_height: _",
                z as "z: _",
                scale,
                content_visible,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM canvas_blocks
            WHERE canvas_id = $1
            ORDER BY z
            "#,
            canvas_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(canvas_blocks)
    }

    async fn get_blocks_in_canvas(&self, canvas_id: Uuid) -> Result<Vec<BlockInCanvas>> {
        self.check_canvas_exists(canvas_id).await?;

        let blocks_in_canvas = sqlx::query_as!(
            BlockInCanvas,
            r#"
            SELECT
                cb.id as "id: _",
                cb.canvas_id as "canvas_id: _",
                cb.grid_x as "grid_x: _",
                cb.grid_y as "grid_y: _",
                cb.grid_width as "grid_width: _",
                cb.grid_height as "grid_height: _",
                cb.z as "z: _",
                cb.scale,
                cb.content_visible,
                cb.created_at as "created_at: _",
                cb.updated_at as "updated_at: _",
                b.id as "block_id: _",
                b.title as block_title,
                b.content as block_content
            FROM canvas_blocks cb
            JOIN blocks b ON cb.block_id = b.id
            WHERE cb.canvas_id = $1
            ORDER BY cb.z
            "#,
            canvas_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(blocks_in_canvas)
    }

    async fn create(&self, input: CanvasBlockCreate) -> Result<CanvasBlock> {
        self.check_canvas_exists(input.canvas_id).await?;
        self.check_block_exists(input.block_id).await?;

        let id = Uuid::new_v4();
        let now = Utc::now();
        let z = 0;
        let scale = 1.0;
        let content_visible = true;

        let canvas_block = sqlx::query_as!(
            CanvasBlock,
            r#"
            INSERT INTO canvas_blocks
                (id, canvas_id, block_id, grid_x, grid_y, grid_width, grid_height, z, scale, content_visible, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING
                id as "id: _",
                canvas_id as "canvas_id: _",
                block_id as "block_id: _",
                grid_x as "grid_x: _",
                grid_y as "grid_y: _",
                grid_width as "grid_width: _",
                grid_height as "grid_height: _",
                z as "z: _",
                scale,
                content_visible,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            "#,
            id,
            input.canvas_id,
            input.block_id,
            input.grid_x,
            input.grid_y,
            input.grid_width,
            input.grid_height,
            z,
            scale,
            content_visible,
            now,
          )
          .fetch_one(&self.pool)
          .await?;

        Ok(canvas_block)
    }

    async fn update(&self, id: Uuid, input: CanvasBlockUpdate) -> Result<CanvasBlock> {
        let now = Utc::now();

        let canvas_block = sqlx::query_as!(
            CanvasBlock,
            r#"
            UPDATE canvas_blocks SET
                grid_x = COALESCE($2, grid_x),
                grid_y = COALESCE($3, grid_y),
                grid_width = COALESCE($4, grid_width),
                grid_height = COALESCE($5, grid_height),
                z = COALESCE($6, z),
                scale = COALESCE($7, scale),
                content_visible = COALESCE($8, content_visible),
                updated_at = $9
            WHERE id = $1
            RETURNING
                id as "id: _",
                canvas_id as "canvas_id: _",
                block_id as "block_id: _",
                grid_x as "grid_x: _",
                grid_y as "grid_y: _",
                grid_width as "grid_width: _",
                grid_height as "grid_height: _",
                z as "z: _",
                scale,
                content_visible,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            "#,
            id,
            input.grid_x,
            input.grid_y,
            input.grid_width,
            input.grid_height,
            input.z,
            input.scale,
            input.content_visible,
            now
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| CanvasBlockError::NotFound { id })?;

        Ok(canvas_block)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM canvas_blocks WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CanvasBlockError::NotFound { id });
        }

        Ok(())
    }
}

impl SqliteCanvasBlockRepository {
    async fn check_canvas_exists(&self, canvas_id: Uuid) -> Result<()> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM canvases WHERE id = $1)",
            canvas_id
        )
        .fetch_one(&self.pool)
        .await?;

        if exists <= 0 {
            return Err(CanvasBlockError::CanvasNotFound { canvas_id });
        }

        Ok(())
    }

    async fn check_block_exists(&self, block_id: Uuid) -> Result<()> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM blocks WHERE id = $1)",
            block_id
        )
        .fetch_one(&self.pool)
        .await?;

        if exists <= 0 {
            return Err(CanvasBlockError::BlockNotFound { block_id });
        }

        Ok(())
    }
}
