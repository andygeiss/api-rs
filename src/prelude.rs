pub use crate::error::Error;

// Specify errors in error.rs
pub type Result<T> = core::result::Result<T, Error>;

// Implement external trait on external type
pub struct Newtype<T>(pub T);
