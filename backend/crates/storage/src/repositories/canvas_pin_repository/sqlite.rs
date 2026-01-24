use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::error::{CanvasPinError, CanvasPinResult as Result};
use super::traits::CanvasPinRepositoryTrait;
use crate::entities::{CanvasPin, CanvasSummary};
use crate::helpers::sqlx_error_kind_helpers::is_foreign_key_violation;

#[derive(Clone, Debug)]
pub struct SqliteCanvasPinRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCanvasPinRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl CanvasPinRepositoryTrait for SqliteCanvasPinRepository {
    async fn is_pinned(&self, canvas_id: Uuid) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM canvas_pins WHERE canvas_id = $1)",
            canvas_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists > 0)
    }

    async fn pin(&self, canvas_id: Uuid) -> Result<CanvasPin> {
        let now = Utc::now();

        let canvas_pin = sqlx::query_as!(
            CanvasPin,
            r#"
            INSERT INTO canvas_pins (canvas_id, pinned_at)
            VALUES ($1, $2)
            RETURNING
                canvas_id as "canvas_id: _",
                pinned_at as "pinned_at: _"
            "#,
            canvas_id,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_foreign_key_violation(&e) {
                return CanvasPinError::CanvasNotFound { canvas_id };
            }
            CanvasPinError::Database(e)
        })?;

        Ok(canvas_pin)
    }

    async fn unpin(&self, canvas_id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM canvas_pins WHERE canvas_id = $1", canvas_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CanvasPinError::NotFound { canvas_id });
        }

        Ok(())
    }

    async fn get_pinned_canvases(&self) -> Result<Vec<CanvasSummary>> {
        let pinned_summaries = sqlx::query_as!(
            CanvasSummary,
            r#"
            SELECT
                c.id as "id: _",
                c.name,
                c.created_at as "created_at: _",
                c.updated_at as "updated_at: _"
            FROM canvas_pins cp
            JOIN canvases c ON cp.canvas_id = c.id
            ORDER BY cp.pinned_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(pinned_summaries)
    }

    async fn unpin_all(&self) -> Result<()> {
        sqlx::query!("DELETE FROM canvas_pins")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
