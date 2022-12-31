use std::env;

use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

use todo_app_backend::model::Todo;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    println!("fetching todos...");
    sqlx::query("INSERT INTO `todos` (`title`, `due_to`) VALUES ('sample', '2023-01-01 12:34')")
        .execute(&pool)
        .await
        .context("An error occured while executing INSERT query.")?;
    let todos = sqlx::query(
        r"SELECT `id`, `title`, `note`, `due_to`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos`"
    )
    .fetch_all(&pool)
    .await
    .context("An error occured while executing SELECT query")?;
    println!("fetched {} todos.", todos.len());
    for todo in todos {
        let todo = Todo::from_row(&todo)
            .with_context(|| format!("Failed to parse fetched row {:?}", todo))?;
        println!("{:?}", todo);
    }
    Ok(())
}
