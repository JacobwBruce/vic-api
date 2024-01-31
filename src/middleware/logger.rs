use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn create_logger() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
}
