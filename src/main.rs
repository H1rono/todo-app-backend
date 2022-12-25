use std::env;

use anyhow::Result;
use sqlx::mysql::MySqlPool;

use backend::model::Todo;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    println!("fetching todos...");
    let todos = sqlx::query_as!(
        Todo,
        r"SELECT `id`, `title`, `note`, `due_to`, `done`, `created_at`, `updated_at`, `deleted_at` FROM `todos`"
    )
    .fetch_all(&pool)
    .await?;
    println!("fetched {} todos.", todos.len());
    for todo in todos {
        println!("{:?}", todo);
    }
    Ok(())
}
