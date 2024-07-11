use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Token {
    pub value: String,
}

#[derive(Clone, Deserialize)]
pub struct User {
    pub name: String,
}

pub async fn authorize(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let mut user_name = "".to_string();
    let mut token_value = "".to_string();
    if let Some(value) = req.headers().get(header::AUTHORIZATION) {
        token_value = value.to_str().unwrap().to_string();
        println!("authorize: token {:?}", token_value);
    }
    /* Block API requests without a token
    if req.uri().path().starts_with("/api") && req.extensions().get::<User>().is_none() {
        return Err(StatusCode::UNAUTHORIZED);
        } */
    req.extensions_mut().insert(Token { value: token_value });
    req.extensions_mut().insert(User { name: user_name });
    Ok(next.run(req).await)
}
