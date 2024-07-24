pub use crate::error::Error;
pub use async_trait::async_trait;

// Specify errors in error.rs
pub type Result<T> = core::result::Result<T, Error>;
