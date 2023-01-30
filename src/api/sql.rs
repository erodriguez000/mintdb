#![allow(unused)]
use crate::db::DS;
use crate::err::Error;
use crate::models::http::sql::SQLRequest;
use crate::models::{Clients, Result};
use mintdb::fs::log::write::log_event;
use mintdb::prelude::f;
use serde_json::{json, Value};
use warp::{
    reply::json,
    ws::Message,
    Reply,
};
use mintdb::db::exe::SQL;
impl warp::reject::Reject for Error {}

pub async fn handler(body: SQL, clients: Clients) -> Result<impl Reply> {
    log_event(&f!("REQUEST >> Body: '{body:?}'")).await.unwrap_or(());
    let db = DS.get().unwrap();
    match db.execute(&body).await {
        Ok(res) => {
            if let Some(topic) = parse_topic(&body) {
                let message = serde_json::to_string(&res).unwrap();
                publish(clients, body.user_id, topic, message).await;
            }
            Ok(json(&json!(res)))
        }
        Err(e) => Ok(json(&json!({"error": e.to_string()})))
    }
}
fn parse_topic(sql: &SQL) -> Option<String> {
    if sql.stmt == "SELECT" || sql.stmt == "ADD" || sql.stmt == "INFO" || sql.stmt == "FIND" || sql.stmt == "MATCH" {
        return None
    }
    if let Value::String(key) = &sql.data["key"] {
        let topic = f!("{}:{}", sql.doc, key);
        return Some(topic)
    } else if sql.doc.len() == 0 {
        let topic = f!("{}", sql.tb);
        return Some(topic)
    } else {
        let topic = f!("{}:{}", sql.tb, sql.doc);
        return Some(topic)
    }
}
async fn publish(clients: Clients, user_id: Option<usize>, topic: String, message: String) {
    clients
        .read()
        .await
        .iter()
        // Return the client with sender user ID
        // .filter(|(_, client)| match user_id {
        //     Some(v) => client.user_id == v,
        //     None => true,
        // })
        // Check if the topics are being subscribed to by the Clients
        .filter(|(_, client)| client.topics.iter().any(|t| topic.starts_with(t)))
        // For each Client subscribed echo the body.message
        .for_each(|(_, client)| {
            // If the client has a sender
            if let Some(sender) = &client.sender {
                // Send body.message
                let _ = sender.send(Ok(Message::text(message.clone())));
            }
        });
}