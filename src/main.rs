use std::env;

use anyhow::{Context, Result};

use todo_app_backend::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url =
        env::var("DATABASE_URL").context("Failed to get environment variable DATABASE_URL")?;
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").context("Failed to get environment variable ALLOWED_ORIGIN")?;
    let port = env::var("PORT").context("Failed to get environment variable PORT")?;
    let port: u16 = port
        .parse()
        .with_context(|| format!("Cannot parse \"{port}\" as u16"))?;
    let conf = Config {
        database_url,
        port,
        allowed_origin,
    };
    todo_app_backend::start(conf)
        .await
        .context("Failed to start server")?;
    Ok(())
}
