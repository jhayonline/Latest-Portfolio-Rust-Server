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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db = database::connect(&database_url).await?;
    let db_middleware = affix_state::inject(db);

    let router = routes::build_router().hoop(db_middleware);

    let port = env::var("PORT").unwrap_or_else(|_| "5001".to_string());
    let addr = format!("0.0.0.0:{port}");

    info!("Salvo server listening on http://{addr}");

    let acceptor = TcpListener::new(&addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
