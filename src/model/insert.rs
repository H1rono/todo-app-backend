use anyhow::Context;

use super::{Database, PartialTodo, Result, Todo};

fn try_cvt(n: u64) -> Result<u32> {
    let res = n
        .try_into()
        .with_context(|| format!("Failed to convert {n} into u32"))?;
    Ok(res)
}

impl Database {
    pub async fn insert_todo(&self, todo: &Todo) -> Result<u32> {
        let Todo {
            id,
            title,
            note,
            due_to,
            done,
            created_at,
            updated_at,
            deleted_at,
        } = todo;
        let id = sqlx::query(
            "INSERT INTO `todos` (`id`, `title`, `note`, `due_to`, `done`, `created_at`, `updated_at`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(title)
        .bind(note)
        .bind(due_to)
        .bind(done)
        .bind(created_at)
        .bind(updated_at)
        .bind(deleted_at)
        .execute(&self.pool)
        .await
        .with_context(|| format!("Failed to INSERT a todo {todo:?}"))?
        .last_insert_id();
        try_cvt(id)
    }

    pub async fn insert_partial_todo(&self, todo: &PartialTodo) -> Result<u32> {
        let PartialTodo {
            title,
            due_to,
            note,
            done,
        } = todo;
        let id = sqlx::query(
            "INSERT INTO `todos` (`title`, `note`, `due_to`, `done`) VALUES (?, ?, ?, ?)",
        )
        .bind(title)
        .bind(note)
        .bind(due_to)
        .bind(*done as i8)
        .execute(&self.pool)
        .await
        .with_context(|| format!("Failed to INSERT a todo with {todo:?}"))?
        .last_insert_id();
        try_cvt(id)
    }
}
