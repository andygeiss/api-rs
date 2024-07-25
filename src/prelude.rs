pub use crate::error::Error;
use std::sync::{Arc, Mutex};

// Support #[automock] for traits
pub use mockall::predicate::*;
pub use mockall::*;

// Specify errors in error.rs
pub type Result<T> = core::result::Result<T, Error>;

// Type alias for a thread-safe trait implementation
pub type ThreadSafe<T> = Arc<Mutex<T>>;

pub fn thread_safe<T>(t: T) -> ThreadSafe<T> {
    Arc::new(Mutex::new(t))
}
