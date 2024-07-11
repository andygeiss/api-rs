use crate::middleware::User;
use askama::Template;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "home.html")]
struct Data {
    user_name: String,
}

pub async fn show_home(Extension(user): Extension<User>) -> impl IntoResponse {
    let user_name = user.name;
    let template = Data { user_name };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
