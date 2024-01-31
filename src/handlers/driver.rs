use crate::models::driver::Driver;
use crate::repositories::drivers_repo::DriversRepository;
use axum::{extract, http};

use axum::routing::{get, Router};

#[derive(serde::Serialize)]
struct ErrorResponse {
    message: String,
}

async fn get_drivers(
    extract::State(db): extract::State<DriversRepository>,
) -> Result<
    (http::StatusCode, axum::Json<Vec<Driver>>),
    (http::StatusCode, axum::Json<ErrorResponse>),
> {
    let drivers = db.get_all_drivers().await;
    match drivers {
        Ok(drivers) => Ok((http::StatusCode::OK, axum::Json(drivers))),
        Err(_) => {
            let response = ErrorResponse {
                message: "Internal Server Error".to_string(),
            };
            Err((
                http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response),
            ))
        }
    }
}

pub fn drivers_router<S>(db: &DriversRepository) -> Router<S> {
    Router::new()
        .route("/", get(get_drivers))
        .with_state(db.clone())
}
