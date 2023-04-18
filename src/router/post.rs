use std::sync::Arc;

use axum::{extract::State, Json};
use hyper::StatusCode;

use crate::model::{PartialTodo, Todo};

use super::{App, AppError};

/// POST /todos
/// body: Json<PartialTodo>
pub async fn post_todo(
    State(app): State<Arc<App>>,
    Json(payload): Json<PartialTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let id = app
        .db
        .insert_partial_todo(&payload)
        .await
        .map_err(AppError::DBErr)?;
    let todo = app.db.fetch_todo_by_id(id).await.map_err(AppError::DBErr)?;
    Ok((StatusCode::CREATED, Json(todo)))
}
