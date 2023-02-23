use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing, Json, Router,
};
use hyper::StatusCode;

use crate::model::{DBError, Todo};

use super::App;

/// GET /todos
async fn get_all_todos(
    State(app): State<Arc<App>>,
) -> Result<(StatusCode, Json<Vec<Todo>>), (StatusCode, &'static str)> {
    let todos = app.db.fetch_all_todos().await.map_err(|e| match e {
        DBError::MySqlError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong while communicating with database",
        ),
        DBError::RowNotFound(_) => unreachable!(),
    })?;
    Ok((StatusCode::ACCEPTED, Json(todos)))
}

/// GET /todos/:id
async fn get_todo_by_id(
    State(app): State<Arc<App>>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let todo = app.db.fetch_todo_by_id(id).await.map_err(|e| match e {
        DBError::MySqlError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong while communicating with database"),
        ),
        DBError::RowNotFound(i) => (StatusCode::NOT_FOUND, format!("No row found with id {i}")),
    })?;
    Ok((StatusCode::ACCEPTED, Json(todo)))
}

pub fn make_router() -> Router<Arc<App>> {
    Router::new()
        .route("/todos", routing::get(get_all_todos))
        .route("/todos/:id", routing::get(get_todo_by_id))
}
