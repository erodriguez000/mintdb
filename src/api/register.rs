use uuid::Uuid;
use warp::{
    http::StatusCode, 
    reply::json, Reply
};

use crate::models::http::register::{RegisterRequest, RegisterResponse};
use crate::models::{client::Client, Clients, Result};

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    // Get RegisterRequest from POST body
    let user_id = body.user_id;
    // Create uuid for user
    let uuid = Uuid::new_v4().as_simple().to_string();
    // Register client
    register_client(uuid.clone(), user_id, clients).await;
    // Forward to websocket url
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

pub async fn register_client(id: String, user_id: usize, clients: Clients) {
    // Insert Client into Clients HashMap
    clients.write().await.insert(
        id,
        Client {
            user_id,
            topics: vec![],
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    // Remove Client from Clients HashMap
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}