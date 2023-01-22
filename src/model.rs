use anyhow::{Context, Result};
use sqlx::{
    types::chrono::{DateTime, Utc},
    MySqlPool,
};

mod delete;
mod fetch;
mod insert;

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
}
