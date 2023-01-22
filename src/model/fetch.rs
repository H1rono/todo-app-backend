use anyhow::{Context, Result};
use sqlx::FromRow;

use super::{Database, Todo};

fn todo_from_row(todo: sqlx::mysql::MySqlRow) -> Result<Todo> {
    Todo::from_row(&todo).with_context(|| format!("Failed to parse fetched row {:?}", todo))
}

impl Database {
    pub async fn fetch_all_todos(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos`")
            .fetch_all(&self.pool)
            .await
            .context("Failed to execute SELECT query")?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    pub async fn fetch_todo_by_id(&self, id: u32) -> Result<Todo> {
        let todo = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `id` = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .with_context(|| format!("Failed to fetch a todo by id {}", id))?;
        todo_from_row(todo)
    }

    pub async fn fetch_todos_like_title(&self, title: &str) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `title` LIKE ?")
            .bind(title)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos where its title like {}", title))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    pub async fn fetch_todos_by_done(&self, done: bool) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `done` = ?")
            .bind(done as i8)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos where its done = {}", done))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }
}