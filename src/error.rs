use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("FromUtf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("Generic error: {0}")]
    Generic(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Paseto error: {0}")]
    Paseto(#[from] rusty_paseto::generic::GenericBuilderError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Tracing error: {0}")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),
}
