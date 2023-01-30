pub use crate::err::Error;
pub use std::format as f;
pub type Result<T> = core::result::Result<T, Error>;