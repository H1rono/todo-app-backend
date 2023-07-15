use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;

use crate::model::Todo;

use super::{App, AppError, Result};

impl App {
    pub async fn todos(&self) -> Result<Vec<Todo>> {
        self.db.fetch_all_todos().await.map_err(AppError::DBErr)
    }

    pub async fn todo_at(&self, id: u32) -> Result<Todo> {
        self.db.fetch_todo_by_id(id).await.map_err(AppError::DBErr)
    }

    /// GET /todos
    pub async fn get_all_todos(
        State(app): State<Arc<App>>,
    ) -> Result<(StatusCode, Json<Vec<Todo>>)> {
        let todos = app.todos().await?;
        Ok((StatusCode::ACCEPTED, Json(todos)))
    }

    /// GET /todos/:id
    pub async fn get_todo_by_id(
        State(app): State<Arc<App>>,
        Path(id): Path<u32>,
    ) -> Result<(StatusCode, Json<Todo>)> {
        let todo = app.todo_at(id).await?;
        Ok((StatusCode::ACCEPTED, Json(todo)))
    }
}
