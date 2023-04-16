use anyhow::Context;

use super::{Database, PartialTodo, Result, TimeStamp, Todo};

impl Database {
    pub async fn update_todo_col_title(&self, id: u32, value: &str) -> Result<Todo> {
        let mut todo = self.fetch_todo_by_id(id).await?;
        sqlx::query("UPDATE `todos` SET `title` = ? WHERE `id` = ?")
            .bind(value)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!("Failed to UPDATE a row where id = {id} with title = {value}")
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
                format!("Failed to UPDATE a row where id = {id} with note = {value}")
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
                format!("Failed to UPDATE a row where id = {id} with due_to = {value}")
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
                format!("Failed to UPDATE a row where id = {id} with done = {value}")
            })?;
        todo.done = value;
        Ok(todo)
    }

    // value: true => SET deleted_at = current_timestamp, false => SET deleted_at = NULL
    pub async fn update_todo_col_delete(&self, id: u32, value: bool) -> Result<Todo> {
        let _ = self.fetch_todo_by_id(id).await?;
        let value = if value { "CURRENT_TIMESTAMP" } else { "NULL" };
        let query = format!("UPDATE `todos` SET `deleted_at` = {value} WHERE `id` = ?");
        sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!("Failed to UPDATE a row where id = {id} with deleted_at = {value}")
            })?;
        self.fetch_todo_by_id(id).await
    }

    pub async fn update_todo_partial(&self, id: u32, value: PartialTodo) -> Result<Todo> {
        let _ = self.fetch_todo_by_id(id).await?;
        sqlx::query(
            "UPDATE `todos` SET `title` = ?, `note` = ?, `due_to` = ?, `done` = ? WHERE `id` = ?",
        )
        .bind(&value.title)
        .bind(&value.note)
        .bind(value.due_to)
        .bind(value.done as i8)
        .bind(id)
        .execute(&self.pool)
        .await
        .with_context(|| {
            format!("Failed to UPDATE a row where id = {id} with partial todo {value:?}")
        })?;
        self.fetch_todo_by_id(id).await
    }
}
