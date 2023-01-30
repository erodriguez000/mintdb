use std::{collections::HashMap, convert::Infallible, sync::Arc};
use warp::Filter;
use tokio::sync::RwLock;
use crate::err::Error;
use crate::models::Clients;

mod auth;
mod health;
mod publish;
mod register;
mod sql;
mod ui;
mod ws;

pub async fn init() -> Result<(), Error> {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let routes = ui::config()
        .or(auth::config())
        .or(sql::config(clients.clone()))
        .or(ws::config(clients.clone()))
        .or(register::config(clients.clone()))
        .or(publish::config(clients.clone()))
        .or(health::config())
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type", "authorization"]).allow_methods(vec!["GET", "POST"]));

    let (adr, srv) =
        warp::serve(routes)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 8000), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen to shutdown signal");
        });

    println!("...Started web server on {}...", &adr);
    println!("\x1b[38;5;50m...Started admin console on http://{}/...\x1b[0m", &adr);
    println!("...Started sql api on http://{}/sql...", &adr);
    println!("\x1b[38;5;50m...Started publish api on http://{}/publish...\x1b[0m", &adr);
    println!("...Started websocket server server on ws://{}/ws...", &adr);
    println!("\x1b[38;5;50m...Started health check endpoint http://{}/health...\x1b[0m", &adr);

    srv.await;
    Ok(())
}

// Adds Clients HashMap as data for routes
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
