use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "home.html")]
struct Data {}

pub async fn read() -> impl IntoResponse {
    let template = Data {};
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
