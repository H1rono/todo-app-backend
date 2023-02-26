use std::{error, fmt, sync::Arc};

use axum::{response::IntoResponse, routing, Router};
use hyper::StatusCode;

use crate::model::{DBError, Database};

mod delete;
mod get;
mod patch;
mod post;

pub struct App {
    pub db: Database,
}

impl App {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let db = Database::connect(url).await?;
        Ok(Self { db })
    }
}

#[derive(Debug)]
pub enum AppError {
    DBErr(DBError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DBErr(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DBErr(DBError::MySqlError(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong while communicating with database",
            )
                .into_response(),
            Self::DBErr(DBError::RowNotFound(i)) => {
                (StatusCode::NOT_FOUND, format!("no todo found for id = {i}")).into_response()
            }
        }
    }
}

// GET /todos
// GET /todos/:id
// POST /todos
// PATCH /todos/:id
// DELETE /todos/:id

pub fn make_router(db: Database) -> Router {
    let app = App::new(db);
    Router::new()
        .route("/todos", routing::get(get::get_all_todos))
        .route("/todos/:id", routing::get(get::get_todo_by_id))
        .route("/todos", routing::post(post::post_todo))
        .route("/todos/:id", routing::delete(delete::delete_todo_by_id))
        .with_state(Arc::new(app))
}
