use axum::routing::get;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(db)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    info!("Starting server");

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
