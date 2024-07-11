use crate::{
    middleware::authorize,
    routes::{home, index, sign_in},
};
use axum::{middleware, routing::get, Router};
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn service() -> Router {
    Router::new()
        .route("/home", get(home::default))
        .route_layer(middleware::from_fn(authorize))
        .route("/sign_in", get(sign_in::default))
        .route("/", get(index::default).post(index::parse_form))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback_service(ServeDir::new("assets"))
}
