use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "sign_in.html")]
struct Data {}

pub async fn show_sign_in() -> impl IntoResponse {
    let template = Data {};
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
