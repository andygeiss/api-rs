use crate::routes::{home, index};
use axum::{routing::get, Router};
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn service() -> Router {
    Router::new()
        .route("/", get(index::read))
        .route("/home", get(home::read))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback_service(ServeDir::new("assets"))
}
