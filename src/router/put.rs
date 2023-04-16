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
pub struct PutTodo {
    title: Option<String>,
    note: Option<String>,
    due_to: Option<TimeStamp>,
    done: Option<bool>,
    deleted: Option<bool>,
}

// PUT /todos/:id
pub async fn put_todo(
    State(app): State<Arc<App>>,
    Path(id): Path<u32>,
    Json(put): Json<PutTodo>,
) -> Result<(StatusCode, &'static str), AppError> {
    let PutTodo {
        title,
        note,
        due_to,
        done,
        deleted,
    } = put;
    let todo = app.db.fetch_todo_by_id(id).await.map_err(AppError::DBErr)?;
    let np = PartialTodo {
        title: title.unwrap_or(todo.title),
        note: note.unwrap_or(todo.note),
        due_to: due_to.unwrap_or(todo.due_to),
        done: done.unwrap_or(todo.done),
    };
    app.db
        .update_todo_partial(id, np)
        .await
        .map_err(AppError::DBErr)?;
    if let Some(d) = deleted {
        app.db
            .update_todo_col_delete(id, d)
            .await
            .map_err(AppError::DBErr)?;
    }
    Ok((StatusCode::OK, "todo updated"))
}
