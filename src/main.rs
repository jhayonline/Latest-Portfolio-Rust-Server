mod database;
mod entity;
mod handlers;
mod routes;

use salvo::cors::{AllowHeaders, AllowMethods, Cors};
use salvo::http::Method;
use salvo::prelude::*;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    todo!()
}
