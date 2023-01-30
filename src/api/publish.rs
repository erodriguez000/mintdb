use warp::{
    http::StatusCode, 
    ws::{Message}, 
    Reply
};
use crate::{ 
    models::Clients, 
    models::event::Event,
    models::Result, 
};

pub async fn handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        // Return the client with sender user ID
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        // Check if the topics are being subscribed to by the Clients
        .filter(|(_, client)| client.topics.contains(&body.topic))
        // For each Client subscribed echo the body.message
        .for_each(|(_, client)| {
            // If the client has a sender
            if let Some(sender) = &client.sender {
                // Send body.message
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });
    Ok(StatusCode::OK)
}