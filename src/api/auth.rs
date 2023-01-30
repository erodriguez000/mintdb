use crate::db::DS;
use crate::models::Result;
use mintdb::fs::log::write::log_event;
use mintdb::prelude::f;
use serde::Deserialize;
use serde_json::json;
use warp::{
    reply::json,
    Reply,
};
#[allow(unused)]
use crate::err::Error;
#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    pub event: String,
    pub username: String,
    pub password: String,
}

pub async fn handler(header: Option<String>, body: AuthRequest) -> Result<impl Reply> {
    println!("{header:?}");
    log_event(&f!("{} >> {}", &body.event, &body.username)).await.unwrap_or(());
    match body.event.as_str() {
        "signup" => {
            let db = DS.get().unwrap();
            match db.sign_up(&body.username, &body.password).await {
                Ok(res) => Ok(json(&json!(res))),
                Err(e) => {
                    Ok(json(&json!({"error": e.to_string()})))
                }
            }
        }
        "signin" => {
            let db = DS.get().unwrap();
            match db.sign_in(&body.username, &body.password).await {
                Ok(res) => Ok(json(&json!(res))),
                Err(e) => {
                    Ok(json(&json!({"error": e.to_string()})))
                }
            }
        }
        "signout" => {
            // let jwt = header.ok_or(Error::InvalidAuth)?;
            let db = DS.get().unwrap();
            match db.sign_out(&body.username).await {
                Ok(_) => Ok(json(&json!("Signed Out"))),
                Err(e) => {
                    Ok(json(&json!({"error": e.to_string()})))
                }
            }
        }
        _ => {
            let res = json!({"error": "ERROR: STMT NOT RECOGNIZED"});
            Ok(json(&res))
        }
    }
}