use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;

use crate::model::{PartialTodo, TimeStamp};

use super::{App, AppError};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PatchTodo {
    title: Option<String>,
    note: Option<String>,
    due_to: Option<TimeStamp>,
    done: Option<bool>,
}

fn merge_into_partial_todo(p_todo: PartialTodo, patch: PatchTodo) -> PartialTodo {
    PartialTodo {
        title: patch.title.unwrap_or(p_todo.title),
        note: patch.note.unwrap_or(p_todo.note),
        due_to: patch.due_to.unwrap_or(p_todo.due_to),
        done: patch.done.unwrap_or(p_todo.done),
    }
}

// PATCH /todos/:id
pub async fn patch_todo(
    State(app): State<Arc<App>>,
    Path(id): Path<u32>,
    Json(patch): Json<PatchTodo>,
) -> Result<(StatusCode, &'static str), AppError> {
    let todo = app.db.fetch_todo_by_id(id).await.map_err(AppError::DBErr)?;
    let np = merge_into_partial_todo(todo.into(), patch);
    app.db
        .update_todo_partial(id, np)
        .await
        .map_err(AppError::DBErr)?;
    Ok((StatusCode::OK, "todo updated"))
}
