use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    MySqlPool,
};

mod delete;
mod fetch;
mod insert;
mod update;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("db/migrations");

pub type TimeStamp = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialTodo {
    pub title: String,
    pub note: String,
    pub due_to: TimeStamp,
    pub done: bool,
}

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let pool = MySqlPool::connect(url)
            .await
            .with_context(|| format!("Failed to connect database {url}"))?;
        Ok(Self::new(pool))
    }

    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug)]
pub enum DBError {
    RowNotFound(u32),
    MySqlError(anyhow::Error),
}

impl From<anyhow::Error> for DBError {
    fn from(value: anyhow::Error) -> Self {
        DBError::MySqlError(value)
    }
}

impl From<DBError> for anyhow::Error {
    fn from(val: DBError) -> Self {
        match val {
            DBError::RowNotFound(id) => anyhow::anyhow!("Row Not found for id {id}"),
            DBError::MySqlError(err) => err,
        }
    }
}

pub type Result<T, E = DBError> = anyhow::Result<T, E>;
