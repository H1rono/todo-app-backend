use std::sync::Arc;

use axum::extract::{Path, State};
use hyper::StatusCode;

use crate::model::DBError;

use super::App;

// DELETE /todos/:id
pub async fn delete_todo_by_id(
    State(app): State<Arc<App>>,
    Path(id): Path<u32>,
) -> (StatusCode, &'static str) {
    let res = app.db.delete_todo(id).await;
    match res {
        Ok(_) => (StatusCode::OK, "the todo was completely deleted"),
        Err(DBError::RowNotFound(_)) => (StatusCode::NOT_FOUND, "no todos matched for the id"),
        Err(DBError::MySqlError(_)) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
    }
}
