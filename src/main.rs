mod handlers;
mod middleware;
mod models;
mod repositories;

use axum::{http, middleware::from_fn, routing::get};
use handlers::driver::drivers_router;
use repositories::drivers_repo::DriversRepository;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::middleware::{auth::auth, logger::create_logger};

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // only checking for the presence of the env var, do not need to store in a variable
    std::env::var("ACCESS_KEY").expect("missing ACCESS_KEY env");

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let drivers_repo = DriversRepository {
        db: Box::leak(Box::new(db)),
    };

    let app = axum::Router::new()
        .route("/health", get(health))
        .nest("/drivers", drivers_router(&drivers_repo))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(create_logger())
        .layer(from_fn(auth));

    info!("Starting server");

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
