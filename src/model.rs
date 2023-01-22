use anyhow::{Context, Result};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow, MySqlPool,
};

pub type TimeStamp = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub note: String,
    pub due_to: TimeStamp,
    pub done: i8,
    pub created_at: TimeStamp,
    pub updated_at: TimeStamp,
    pub deleted_at: Option<TimeStamp>,
}

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self> {
        let pool = MySqlPool::connect(url)
            .await
            .with_context(|| format!("Failed to connect database {}", url))?;
        Ok(Self { pool })
    }

    pub async fn fetch_all_todos(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos`")
            .fetch_all(&self.pool)
            .await
            .context("Failed to execute SELECT query")?;
        let mut res = vec![];
        for todo in todos {
            let todo = Todo::from_row(&todo)
                .with_context(|| format!("Failed to parse fetched row {:?}", todo))?;
            res.push(todo);
        }
        Ok(res)
    }

    pub async fn fetch_todo_by_id(&self, id: u32) -> Result<Todo> {
        let todo = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `id` = ? LIMIT 1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .with_context(|| format!("Failed to fetch a todo by id {}", id))?;
        Todo::from_row(&todo).with_context(|| format!("Failed to parse fetched row {:?}", todo))
    }

    pub async fn fetch_todos_like_title(&self, title: &str) -> Result<Vec<Todo>> {
        let todos = sqlx::query("SELECT `id`, `title`, `note`, `due_to`, `created_at`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `title` LIKE ?")
            .bind(title)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("Failed to SELECT todos where its title like {}", title))?
            .into_iter()
            .map(|t| {
                Todo::from_row(&t).with_context(|| format!("Failed to parse fetched row {:?}", t))
            })
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    pub async fn insert_todo(&self, todo: Todo) -> Result<()> {
        let Todo {
            id,
            title,
            note,
            due_to,
            done,
            created_at,
            updated_at,
            deleted_at,
        } = todo.clone();
        sqlx::query("INSERT INTO `todos` (`id`, `title`, `note`, `due_to`, `done`, `created_at`, `updated_at`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
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
            .with_context(|| format!("Failed to INSERT a todo {:?}", todo))?;
        Ok(())
    }

    pub async fn insert_partial_todo(
        &self,
        title: String,
        due_to: TimeStamp,
        note: Option<String>,
    ) -> Result<()> {
        let note = note.unwrap_or_default();
        sqlx::query("INSERT INTO `todos` (`title`, `note`, `due_to`) VALUES (?, ?, ?)")
            .bind(title.clone())
            .bind(note.clone())
            .bind(due_to)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to INSERT a todo with title=`{}`, due_to=`{}`, note=`{}`",
                    title, due_to, note
                )
            })?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: u32) -> Result<()> {
        sqlx::query("DELETE FROM `todos` WHERE `id` = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| format!("Failed to DELETE todo where id={}", id))?;
        Ok(())
    }
}
