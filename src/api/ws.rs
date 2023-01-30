use tokio::sync::mpsc;
use serde_json::from_str;
use futures::{FutureExt, StreamExt};
use warp::{ws::{WebSocket, Message}, Reply};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{models::{client::Client, Clients, Result}, models::http::topic::TopicsRequest};

pub async fn handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    // Called after register (ie Client is in Clients HashMap already)
    // Get client from Clients HashMap
    let client = clients.read().await.get(&id).cloned();
    match client {
        // If client is found, upgrade to websocket connection
        // Call client_connection async closure
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

// Async closure called on upgrade to ws connection
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    // Register client sender and receiver from warp
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    // Register client sender and receiver from mpsc unbounded channel
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    // Set client_rcv as tokio::UnboundedReceiverStream
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    // echo messages from the unbound client_rcv through client_ws_sender
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("Error sending websocket message: {}", e);
        }
    }));

    // Register the client_sender unbound sender as the Client struct's sender
    client.sender = Some(client_sender);
    
    // Insert new client into Clients HashMap struct
    clients.write().await.insert(id.clone(), client);

    // Client is connected
    println!("{} connected", id);

    // While websocket messages are being received
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}, {}", id.clone(), e);
                break;
            }
        };
        // Handle received message
        client_msg(&id, msg, &clients).await;
    }

    // Handle disconnection
    clients.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", id, msg);
    // Validate received message
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    // If message is "ping" return, no need to respond
    if message == "ping" || message == "ping\n" {
        return;
    }
    // Validate topics request (deserialize to struct from str message)
    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error while parsing message to topics request: {}", e);
            return;
        }
    };

    let mut locked = clients.write().await;
    // Add requested topics to user Client struct
    if let Some(v) = locked.get_mut(id) {
        v.topics = topics_req.topics;
    }
}