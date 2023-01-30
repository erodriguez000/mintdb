use warp::Filter;
use crate::api::health;

pub fn config() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health").and_then(health::handler)
}