pub mod auth;
pub mod db;
mod table;
mod doc;
mod tx;
mod graph;
pub mod dev;
pub mod err;
pub mod fs;
pub mod kvs;
pub mod prelude;
pub mod rule;
pub mod schema;
pub mod util;

pub use crate::err::Error;
pub use std::format as f;
pub type Result<T> = core::result::Result<T, Error>;
pub use kvs::store::Datastore;