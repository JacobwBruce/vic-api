use crate::models::driver::Driver;
use crate::repositories::drivers_repo::{DriverError, DriversRepository};
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

async fn get_driver_by_phone_number(
    extract::Path((phone_number,)): extract::Path<(String,)>,
    extract::State(db): extract::State<DriversRepository>,
) -> Result<(http::StatusCode, axum::Json<Driver>), (http::StatusCode, axum::Json<ErrorResponse>)> {
    let driver = db.get_driver_by_phone_number(&phone_number).await;
    match driver {
        Ok(driver) => Ok((http::StatusCode::OK, axum::Json(driver))),
        Err(err) => match err.error {
            DriverError::NotFound => {
                let response = ErrorResponse {
                    message: err.message,
                };
                return Err((http::StatusCode::NOT_FOUND, axum::Json(response)));
            }
            _ => {
                let response = ErrorResponse {
                    message: "Internal Server Error".to_string(),
                };
                return Err((
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(response),
                ));
            }
        },
    }
}

pub fn drivers_router<S>(db: &DriversRepository) -> Router<S> {
    Router::new()
        .route("/", get(get_drivers))
        .route("/:phone_number", get(get_driver_by_phone_number))
        .with_state(db.clone())
}
