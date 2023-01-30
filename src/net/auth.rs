use warp::Filter;
use crate::api::auth;

const HEADER_XAUTH: &str = "Authorization";

pub fn config() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth")
        .and(warp::post())
        .and(warp::header::optional(HEADER_XAUTH))
        // Expecting JSON body serialized from SQLRequest struct
        .and(warp::body::json())
        // Clone Clients HashMap to use as data in route
        // Closure captures: (body: SQLRequest, clients: Clients)
        .and_then(auth::handler)
}