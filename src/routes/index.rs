use crate::{
    security, services::authentication::repositories::AccountRepository, state::SharedState,
};
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct Data {
    token: String,
}

#[derive(Deserialize)]
pub struct SignIn {
    username: String,
    password: String,
}

pub async fn default() -> impl IntoResponse {
    let template = Data {
        token: "".to_string(),
    };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}

pub async fn parse_form(
    State(state): State<SharedState>,
    Form(form): Form<SignIn>,
) -> impl IntoResponse {
    let mut token = "".to_string();
    // Check user credentials
    if let Ok(account) = state
        .account_repository
        .lock()
        .unwrap()
        .read("foo".to_string())
    {
        token = security::create_token();
    }
    // Create a template response
    let template = Data { token };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
