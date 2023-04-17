use std::env;
use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::http::HeaderValue;

use todo_app_backend::{model::Database, router::make_router};

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        env::var("DATABASE_URL").context("Failed to get environment variable DATABASE_URL")?;
    let allowed_origin = env::var("ALLOWED_ORIGIN")
        .context("Failed to get environment variable ALLOWED_ORIGIN")?
        .parse::<HeaderValue>()
        .context("Failed to parse ALLOWED_ORIGIN as HeaderValue")?;
    let db = Database::connect(&url).await?;
    let app_router = make_router(db, allowed_origin);
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
