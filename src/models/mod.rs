use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Rejection;
use self::client::Client;

pub mod client;
pub mod event;
pub mod http;


pub type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;