use warp::{
    Reply,
    http::StatusCode
};
use crate::models::Result;

pub async fn handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}