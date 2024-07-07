use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error("IO {0}")]
    IO(#[from] std::io::Error),
}
