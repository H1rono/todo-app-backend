use std::env;

use anyhow::{Context, Result};
use chrono::DateTime;

use todo_app_backend::model::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        env::var("DATABASE_URL").context("Failed to get environment variable DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    db.insert_partial_todo(
        "sample".to_string(),
        DateTime::parse_from_str("2023-06-01 12:34:00 +0000", "%Y-%m-%d %H:%M:%S %z")?.into(),
        None,
    )
    .await?;
    let todos = db.fetch_all_todos().await?;
    for todo in todos {
        println!("{:?}", todo);
    }
    Ok(())
}
