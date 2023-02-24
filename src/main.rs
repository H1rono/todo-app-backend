use std::env;
use std::net::SocketAddr;

use anyhow::{Context, Result};

use todo_app_backend::{model::Database, router::make_router};

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        env::var("DATABASE_URL").context("Failed to get environment variable DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    let app_router = make_router(db);
    let port = env::var("PORT").context("Failed to get environment variable PORT")?;
    let port: u16 = port
        .parse()
        .with_context(|| format!("Cannot parse \"{port}\" as u16"))?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
        .context("Something went wrong while serving")?;
    Ok(())
}
