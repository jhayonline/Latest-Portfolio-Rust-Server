mod database;
mod entity;
mod handlers;
mod routes;

use salvo::cors::{AllowOrigin, Cors};
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
    let database = database::connect(&database_url).await?;

    let cors = Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .allow_headers("content-type")
        .allow_credentials(false)
        .into_handler();

    let database_middleware = affix_state::inject(database);

    // DB middleware on the router, CORS on the service (runs before route matching)
    let router = routes::build_router().hoop(database_middleware);
    let service = Service::new(router).hoop(cors);

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let address = format!("0.0.0.0:{port}");

    info!("Salvo server listening on http://{address}");

    let acceptor = TcpListener::new(address).bind().await;
    Server::new(acceptor).serve(service).await;

    Ok(())
}
