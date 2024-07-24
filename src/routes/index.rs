use crate::{
    repositories::account_file::AccountFileRepository, security,
    services::authentication::repositories::AccountRepository,
};
use askama::Template;
use axum::{
    extract::Form,
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

pub async fn parse_form(Form(form): Form<SignIn>) -> impl IntoResponse {
    let mut token = "".to_string();
    // Check user credentials
    let path = "./data/accounts.json".to_string();
    let repo = AccountFileRepository::new(path.clone());
    if let Ok(account) = repo.read(form.username).await {
        let password = form.password;
        let password_hash = account.hash;
        if security::password::is_valid(password_hash, password) {
            token = security::token::create();
        }
    }
    // Create a template response
    let template = Data { token };
    let response = template.render().unwrap();
    (StatusCode::OK, Html(response).into_response())
}
