use std::{error, fmt, str::FromStr};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialTodo {
    pub title: String,
    pub note: String,
    pub due_to: TimeStamp,
    pub done: bool,
}

impl PartialTodo {
    pub fn new(title: &str, note: &str, due_to: TimeStamp, done: bool) -> Self {
        Self {
            title: title.to_string(),
            note: note.to_string(),
            due_to,
            done,
        }
    }
}

impl TryFrom<(String, String, String, bool)> for PartialTodo {
    type Error = anyhow::Error;
    fn try_from(
        (title, note, due_to, done): (String, String, String, bool),
    ) -> std::result::Result<Self, Self::Error> {
        let due_to = TimeStamp::from_str(&due_to)
            .with_context(|| format!("Failed to parse string '{due_to}' as timestamp"))?;
        Ok(Self {
            title,
            note,
            due_to,
            done,
        })
    }
}

impl TryFrom<(&str, &str, &str, bool)> for PartialTodo {
    type Error = anyhow::Error;
    fn try_from(
        (title, note, due_to, done): (&str, &str, &str, bool),
    ) -> std::result::Result<Self, Self::Error> {
        let due_to = TimeStamp::from_str(due_to)
            .with_context(|| format!("Failed to parse string '{due_to}' as timestamp"))?;
        Ok(PartialTodo::new(title, note, due_to, done))
    }
}

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

impl From<Todo> for PartialTodo {
    fn from(val: Todo) -> Self {
        Self {
            title: val.title,
            note: val.note,
            due_to: val.due_to,
            done: val.done != 0,
        }
    }
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

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DBError::*;
        match self {
            RowNotFound(i) => write!(f, "no row found with id = {i}"),
            MySqlError(e) => write!(f, "MySql error: {e}"),
        }
    }
}

impl error::Error for DBError {}

impl From<anyhow::Error> for DBError {
    fn from(value: anyhow::Error) -> Self {
        DBError::MySqlError(value)
    }
}

pub type Result<T, E = DBError> = anyhow::Result<T, E>;
