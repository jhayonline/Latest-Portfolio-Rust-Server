use anyhow::Context;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::info;

pub async fn connect(database_url: &str) -> anyhow::Result<DatabaseConnection> {
    let mut opts = ConnectOptions::new(database_url);

    opts.min_connections(2)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(60))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opts)
        .await
        .context("Failed to connect to Postgres")?;

    info!("Database connected");
    Ok(db)
}
