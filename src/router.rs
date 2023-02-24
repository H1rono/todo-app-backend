use std::sync::Arc;

use axum::{routing, Router};

use crate::model::Database;

mod get;

pub struct App {
    pub db: Database,
}

impl App {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let db = Database::connect(url).await?;
        Ok(Self { db })
    }
}

// GET /todos
// GET /todos/:id
// POST /todos
// PATCH /todos/:id
// DELETE /todos/:id

pub fn make_router(db: Database) -> Router {
    let app = App::new(db);
    Router::new()
        .route("/todos", routing::get(get::get_all_todos))
        .route("/todos/:id", routing::get(get::get_todo_by_id))
        .with_state(Arc::new(app))
}
