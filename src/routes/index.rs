use crate::middleware::Session;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

#[derive(Template)]
#[template(path = "index.html")]
struct Data {
    token: String,
}

pub async fn show_index(Extension(session): Extension<Session>) -> impl IntoResponse {
    let token = session.token;
    let template = Data { token };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
