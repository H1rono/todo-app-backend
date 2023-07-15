use std::sync::Arc;

use axum::{extract::State, Json};
use hyper::StatusCode;

use crate::model::{PartialTodo, Todo};

use super::{App, AppError, Result};

impl App {
    pub async fn add_todo(&self, ptodo: PartialTodo) -> Result<Todo> {
        let id = self
            .db
            .insert_partial_todo(&ptodo)
            .await
            .map_err(AppError::DBErr)?;
        self.db.fetch_todo_by_id(id).await.map_err(AppError::DBErr)
    }

    /// POST /todos
    /// body: Json<PartialTodo>
    pub async fn post_todo(
        State(app): State<Arc<App>>,
        Json(payload): Json<PartialTodo>,
    ) -> Result<(StatusCode, Json<Todo>)> {
        let todo = app.add_todo(payload).await?;
        Ok((StatusCode::CREATED, Json(todo)))
    }
}
