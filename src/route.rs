use axum::{routing, Router};

use crate::model::Database;

pub async fn index() -> String {
    "Hello, world!".to_string()
}

pub fn make_router(db: Database) -> Router {
    Router::new().route("/", routing::get(index))
}
