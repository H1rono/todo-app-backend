use std::sync::Arc;

use axum::extract::{Path, State};
use hyper::StatusCode;

use super::{App, AppError, Result};

impl App {
    pub async fn del_todo(&self, id: u32) -> Result<()> {
        self.db
            .delete_todo(id)
            .await
            .map(|_| ())
            .map_err(AppError::DBErr)
    }

    // DELETE /todos/:id
    pub async fn delete_todo_by_id(
        State(app): State<Arc<App>>,
        Path(id): Path<u32>,
    ) -> Result<(StatusCode, &'static str)> {
        app.del_todo(id)
            .await
            .map(|_| (StatusCode::OK, "the todo was completely deleted"))
    }
}
