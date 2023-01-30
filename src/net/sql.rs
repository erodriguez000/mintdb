use warp::Filter;
// use crate::{handlers::sql_handler::sql_handler, models::Clients};
use crate::{models::Clients, api::sql};

use super::with_clients;

pub fn config(clients: Clients) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sql")
        .and(warp::post())
        // Expecting JSON body serialized from SQLRequest struct
        .and(warp::body::json())
        // Clone Clients HashMap to use as data in route
        .and(with_clients(clients.clone()))
        // Closure captures: (body: SQLRequest, clients: Clients)
        .and_then(sql::handler)
}