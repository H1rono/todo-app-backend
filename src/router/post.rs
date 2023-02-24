use std::sync::Arc;

use axum::{extract::State, Json};
use hyper::StatusCode;

use crate::model::{DBError, PartialTodo};

use super::App;

/// POST /todos
/// body: Json<PartialTodo>
pub async fn post_todo(
    State(app): State<Arc<App>>,
    Json(payload): Json<PartialTodo>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    app.db
        .insert_partial_todo(&payload)
        .await
        .map_err(|e| match e {
            DBError::MySqlError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong while communicating with database",
            ),
            DBError::RowNotFound(_) => unreachable!(),
        })?;
    Ok(StatusCode::NO_CONTENT)
}
