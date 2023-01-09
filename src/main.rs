use std::env;
use std::net::SocketAddr;

use anyhow::{Context, Result};

use todo_app_backend::{model::Database, route::make_router};

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        env::var("DATABASE_URL").context("Failed to get environment variable DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    let app = make_router(db);
    let port = env::var("PORT").context("Failed to get environment variable PORT")?;
    let port: u16 = port
        .parse()
        .with_context(|| format!("Cannot parse \"{}\" as u16", port))?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Something went wrong while serving")?;
    Ok(())
}
