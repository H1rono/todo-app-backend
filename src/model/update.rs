use anyhow::Context;

use super::{Database, Result, TimeStamp, Todo};

impl Database {
    pub async fn update_todo_col_title(&self, id: u32, value: &str) -> Result<Todo> {
        let mut todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("UPDATE `todos` SET `title` = ? WHERE `id` = ?")
            .bind(value)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to UPDATE a row where id = {} with title = {}",
                    id, value
                )
            })?;
        todo.title = String::from(value);
        Ok(todo)
    }

    pub async fn update_todo_col_note(&self, id: u32, value: &str) -> Result<Todo> {
        let mut todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("UPDATE `todos` SET `note` = ? WHERE `id` = ?")
            .bind(value)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to UPDATE a row where id = {} with note = {}",
                    id, value
                )
            })?;
        todo.note = String::from(value);
        Ok(todo)
    }

    pub async fn update_todo_col_due(&self, id: u32, value: TimeStamp) -> Result<Todo> {
        let mut todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("UPDATE `todos` SET `due_to` = ? WHERE `id` = ?")
            .bind(value)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to UPDATE a row where id = {} with due_to = {}",
                    id, value
                )
            })?;
        todo.due_to = value;
        Ok(todo)
    }

    pub async fn update_todo_col_done(&self, id: u32, value: bool) -> Result<Todo> {
        let mut todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("UPDATE `todos` SET `done` = ? WHERE `id` = ?")
            .bind(value as i8)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to UPDATE a row where id = {} with done = {}",
                    id, value
                )
            })?;
        todo.done = value as i8;
        Ok(todo)
    }

    // value: true => SET deleted_at = current_timestamp, false => SET deleted_at = NULL
    pub async fn update_todo_col_delete(&self, id: u32, value: bool) -> Result<Todo> {
        let _ = self.fetch_todo_by_id(id).await?;
        let value = if value { "CURRENT_TIMESTAMP" } else { "NULL" };
        let query = format!("UPDATE `todos` SET `deleted_at` = {} WHERE `id` = ?", value);
        sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to UPDATE a row where id = {} with deleted_at = {}",
                    id, value
                )
            })?;
        self.fetch_todo_by_id(id).await
    }
}