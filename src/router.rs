use crate::routes::index;
use axum::{routing::get, Router};
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

pub fn service() -> Router {
    Router::new()
        .route("/", get(index::read))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback_service(ServeDir::new("assets"))
}
