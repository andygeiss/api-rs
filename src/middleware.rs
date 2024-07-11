use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::Deserialize;

use crate::security;

#[derive(Clone, Deserialize)]
pub struct Token {
    pub value: String,
}

pub async fn authorize(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract authorization header
    if let Some(hv) = req.headers().get(header::AUTHORIZATION) {
        let value = hv.to_str().unwrap().to_string();
        // Check the token
        let (_, suffix) = value.split_at(7);
        if security::is_token_valid(suffix.to_string()) {
            req.extensions_mut().insert(Token { value });
        }
        return Ok(next.run(req).await);
    }
    // Block API requests without a token
    Err(StatusCode::UNAUTHORIZED)
}
