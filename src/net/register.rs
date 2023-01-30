use warp::Filter;

use crate::models::Clients;
use crate::api::register::{register_handler, unregister_handler};
use super::with_clients;

pub fn config(clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // initialize /register route
    let register = warp::path("register");
    // Add register routes from /register
    let register_routes = register
        // POST request for /register
        .and(warp::post())
        // Expecting JSON body in Post request handler, serialized from RegisterRequest
        .and(warp::body::json())
        // Clone Clients HashMap to use as data in route
        .and(with_clients(clients.clone()))
        // Call register_handler
        // Closure captures: (body: RegisterRequest, clients: Clients)
        .and_then(register_handler)
        // Register a DELETE route for /register
        .or(register
            // Handles DELETE request for /register
            .and(warp::delete())
            // Receiving the parameters of user_id ie /register/{user_id}
            .and(warp::path::param())
            // Clone Clients HashMap to use as data in route
            .and(with_clients(clients.clone()))
            // Call unregister_handler to remove user from Clients HashMap
            // Closure captures: (id: String, clients: Clients)
            .and_then(unregister_handler));
        register_routes
}