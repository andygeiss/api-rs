use crate::security;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn authorize(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract authorization header
    if let Some(hv) = req.headers().get(header::AUTHORIZATION) {
        let value = hv.to_str().unwrap().to_string();
        // Check the token
        let (_, suffix) = value.split_at(7);
        if security::token::is_valid(suffix.to_string()) {
            return Ok(next.run(req).await);
        }
    }
    // Block API requests without a token
    Err(StatusCode::UNAUTHORIZED)
}
