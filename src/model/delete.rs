use anyhow::Context;

use super::{Database, Result, Todo};

impl Database {
    pub async fn delete_todo(&self, id: u32) -> Result<Todo> {
        let todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("DELETE FROM `todos` WHERE `id` = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| format!("Failed to DELETE todo where id={}", id))?;
        Ok(todo)
    }
}
