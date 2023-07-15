use std::sync::Arc;

use axum::extract::{Path, State};
use hyper::StatusCode;

use super::{App, AppError, Result};

// DELETE /todos/:id
pub async fn delete_todo_by_id(
    State(app): State<Arc<App>>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, &'static str)> {
    app.db.delete_todo(id).await.map_err(AppError::DBErr)?;
    Ok((StatusCode::OK, "the todo was completely deleted"))
}
