use std::{error, fmt, sync::Arc};

use axum::{http::HeaderValue, response::IntoResponse, routing, Router};
use hyper::StatusCode;
use tower_http::cors::{Any, CorsLayer};

use crate::model::{DBError, Database};

mod delete;
mod get;
mod post;
mod put;

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
// PUT /todos/:id
// DELETE /todos/:id

pub fn make_router(db: Database, allowed_origin: HeaderValue) -> Router {
    let app = App::new(db);
    let api = Router::new()
        .route(
            "/todos",
            routing::get(get::get_all_todos).post(post::post_todo),
        )
        .route(
            "/todos/:id",
            routing::get(get::get_todo_by_id)
                .put(put::put_todo)
                .delete(delete::delete_todo_by_id),
        )
        .with_state(Arc::new(app))
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_origin)
                .allow_methods(Any)
                .allow_headers(Any),
        );
    Router::new().nest("/api", api)
}
