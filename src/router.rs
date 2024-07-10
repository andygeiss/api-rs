use crate::{
    middleware::authorize,
    routes::{home::show_home, index::show_index, sign_in::show_sign_in},
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
        .route("/", get(show_index).post(show_index))
        .route("/home", get(show_home))
        .route("/sign_in", get(show_sign_in))
        .route_layer(middleware::from_fn(authorize))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback_service(ServeDir::new("assets"))
}
