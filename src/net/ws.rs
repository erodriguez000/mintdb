use warp::Filter;
use crate::models::Clients;
use crate::api::ws;
use super::with_clients;

pub fn config(clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Register websocket route path /ws
    warp::path("ws")
        // As websocket connection
        .and(warp::ws())
        // Receiving the parameters of user_id ie /ws/{user_id}
        .and(warp::path::param())
        // Clone Clients HashMap to use as data in route
        .and(with_clients(clients.clone()))
        // Call ws_handler to handle websocket session
        // Closure captures: (ws: warp::ws::Ws, id: String, clients: Clients)
        .and_then(ws::handler)
}