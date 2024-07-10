use axum::{
    body::Body,
    extract::{self, Form, FromRequest, Request},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json, RequestExt, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct SignInForm {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
}

#[derive(Clone, Deserialize)]
pub struct Session {
    #[serde(default)]
    pub token: String,
}

pub async fn authorize(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    handle_form(&mut req);
    handle_token(&mut req);
    return Ok(next.run(req).await);
    // Err(StatusCode::UNAUTHORIZED)
}

fn handle_form(req: &mut Request) {
    // Extract form
    let content_type = req
        .headers_mut()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok());
    if let Some(value) = content_type {
        if value.starts_with("application/x-www-form-urlencoded") && req.method().as_str() == "POST"
        {
        }
    }
}

fn handle_token(req: &mut Request) {
    // Extract authorization header
    let authorization = req
        .headers_mut()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    // Add session information as an extension
    let mut token = "".to_string();
    if let Some(value) = authorization {
        token = value.to_string();
    }
    req.extensions_mut().insert(Session { token });
}
