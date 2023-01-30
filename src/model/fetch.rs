use anyhow::Context;
use sqlx::FromRow;

use super::{DBError, Database, Result, Todo};

fn todo_from_row(todo: sqlx::mysql::MySqlRow) -> Result<Todo> {
    let ctx = || format!("Failed to parse fetched row {:?}", todo);
    let res = Todo::from_row(&todo).with_context(ctx)?;
    Ok(res)
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
        let mut todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `id` = ? LIMIT 1")
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to fetch a todo by id {}", id))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        match todos.pop() {
            Some(todo) => Ok(todo),
            None => Err(DBError::RowNotFound(id)),
        }
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

    pub async fn fetch_todos_by_title(&self, title: &str) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `title` = ?")
            .bind(title)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos by title {}", title))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    pub async fn fetch_todos_like_note(&self, note: &str) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `note` LIKE ?")
            .bind(note)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos where its note like {}", note))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    pub async fn fetch_todos_by_note(&self, note: &str) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `note` = ?")
            .bind(note)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos by noteiption {}", note))?
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

    pub async fn fetch_todos_by_deleted(&self, deleted: bool) -> Result<Vec<Todo>> {
        let query = format!(
            "SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `deleted_at` IS {}",
            if deleted { "NOT NULL" } else { "NULL" }
        );
        let todos = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos where deleted = {}", deleted))?
            .into_iter()
            .map(todo_from_row)
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }
}
