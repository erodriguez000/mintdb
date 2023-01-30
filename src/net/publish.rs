use warp::Filter;
use crate::{api::publish, models::Clients};

use super::with_clients;

pub fn config(clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let publish = warp::path!("publish")
            // Expecting JSON body serialized from Event struct
            .and(warp::body::json())
            // Clone Clients HashMap to use as data in route
            .and(with_clients(clients.clone()))
            // Call publish_handler to remove user from Clients HashMap
            // Closure captures: (body: Event, clients: Clients)
            .and_then(publish::handler);
    publish
}