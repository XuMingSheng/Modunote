use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::error::{CanvasError, CanvasResult as Result};
use super::traits::CanvasRepositoryTrait;
use crate::entities::{Canvas, CanvasCreate, CanvasSummary, CanvasUpdate};

#[derive(Clone, Debug)]
pub struct SqliteCanvasRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCanvasRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl CanvasRepositoryTrait for SqliteCanvasRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Canvas>> {
        let canvas = sqlx::query_as!(
            Canvas,
            r#"
            SELECT 
                id as "id: _", 
                name, 
                viewport_x, 
                viewport_y, 
                zoom_level,
                created_at as "created_at: _", 
                updated_at as "updated_at: _"
            FROM canvases WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(canvas)
    }

    async fn create(&self, input: CanvasCreate) -> Result<Canvas> {
        let now = Utc::now();

        let canvas = sqlx::query_as!(
            Canvas,
            r#"
            INSERT INTO canvases 
                (id, name, viewport_x, viewport_y, zoom_level, created_at, updated_at)
                VALUES ($1, $2, 0.0, 0.0, 1.0, $3, $3)
            RETURNING 
                id as "id: _", 
                name, 
                viewport_x, 
                viewport_y, 
                zoom_level,
                created_at as "created_at: _", 
                updated_at as "updated_at: _"
            "#,
            input.id,
            input.name,
            now,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(canvas)
    }

    async fn update_by_id(&self, id: Uuid, input: &CanvasUpdate) -> Result<Canvas> {
        let now = Utc::now();

        let canvas = sqlx::query_as!(
            Canvas,
            r#"
            UPDATE canvases SET 
                name = COALESCE($2, name),
                viewport_x = COALESCE($3, viewport_x),
                viewport_y = COALESCE($4, viewport_y),
                zoom_level = COALESCE($5, zoom_level),
                updated_at = $6
            WHERE id = $1
            RETURNING 
                id as "id: _", 
                name, 
                viewport_x, 
                viewport_y, 
                zoom_level,
                created_at as "created_at: _", 
                updated_at as "updated_at: _"
            "#,
            id,
            input.name,
            input.viewport_x,
            input.viewport_y,
            input.zoom_level,
            now
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| CanvasError::NotFound { id })?;

        Ok(canvas)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM canvases WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CanvasError::NotFound { id });
        }

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<CanvasSummary>> {
        let sql_query_str = format!("%{query}%");

        let canvases = sqlx::query_as!(
            CanvasSummary,
            r#"
            SELECT
                id as "id: _",
                name,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM canvases
            WHERE name LIKE $1
            ORDER BY name
            "#,
            sql_query_str
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(canvases)
    }
}
