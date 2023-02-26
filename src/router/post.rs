use std::sync::Arc;

use axum::{extract::State, Json};
use hyper::StatusCode;

use crate::model::PartialTodo;

use super::{App, AppError};

/// POST /todos
/// body: Json<PartialTodo>
pub async fn post_todo(
    State(app): State<Arc<App>>,
    Json(payload): Json<PartialTodo>,
) -> Result<StatusCode, AppError> {
    app.db
        .insert_partial_todo(&payload)
        .await
        .map_err(AppError::DBErr)?;
    Ok(StatusCode::NO_CONTENT)
}
