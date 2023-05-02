pub mod config;
pub mod model;
pub mod router;

use std::net::SocketAddr;

use anyhow::{Context, Result};

use config::Config;
use model::Database;
use router::make_router;

pub async fn start(conf: Config) -> Result<()> {
    let Config {
        database_url,
        port,
        allowed_origin,
    } = conf;
    let db = Database::connect(&database_url).await?;
    let app_router = make_router(db, allowed_origin.parse()?);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
        .context("Something went wrong while serving")?;
    Ok(())
}
