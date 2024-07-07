use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "index.html")]
struct Data {
    token: String,
}

pub async fn read() -> impl IntoResponse {
    let template = Data {
        token: "".to_string(),
    };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
