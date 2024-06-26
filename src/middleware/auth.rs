use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::error;

pub async fn auth<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // if request is to /health, allow it
    if request.uri().path() == "/health" {
        let response = next.run(request).await;
        return Ok(response);
    }

    if let Some(authorization) = request.headers().get("Authorization") {
        if let Some(auth_str) = authorization.to_str().ok() {
            if auth_str.to_lowercase().starts_with("key ") {
                let token_value = auth_str["key ".len()..].to_string();

                if token_value == get_access_key() {
                    let response = next.run(request).await;
                    return Ok(response);
                }
            }
        }
    }

    error!("Unauthorized request");
    Err(StatusCode::UNAUTHORIZED)
}

fn get_access_key() -> String {
    let key = std::env::var("ACCESS_KEY").expect("missing ACCESS_KEY env");
    key
}
