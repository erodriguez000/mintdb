use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::auth::iam;
use crate::kvs::store::Datastore;
use crate::prelude::*;
use crate::util::time::get_unix_time;
use crate::auth::token::jwt;

#[derive(Serialize, Debug)]
pub struct TokenResponse {
    pub code: i32,
    pub token: String,
    pub status: String,
}
const SECRET: &str = "secret";
impl Datastore {
    pub async fn sign_in(&self, username: &str, password: &str) -> Result<TokenResponse> {
        let user = self.get_one_auth("auth", username).await?;
        if let Some(hashed_pw) = user.get("pwd") {
            let hashed_password = hashed_pw.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect::<Vec<u8>>();
            if iam::verify_password(password, &hashed_password)? {
                let data = json!({
                    "active": true,
                    "last_online": get_unix_time()
                });
                let user = self.merge_auth("auth", username, data).await?;
                if let Value::String(uid) = &user["username"] {
                    let jwt = jwt::encode(uid.into(), SECRET)?;
                    let token = TokenResponse{ code: 200, token: jwt, status: "OK".into() };
                    Ok(token)
                } else {
                    Err(Error::Request)
                }
            } else {
                Err(Error::Request)
            }
        } else {
            Err(Error::Request)
        }
    }
    pub async fn sign_up(&self, username: &str, pwd: &str) -> Result<TokenResponse> {
        let hashed_password = iam::hash_password(pwd)?;
        let data = json!({
            "username": username,
            "pwd": hashed_password,
            "uid": Uuid::new_v4().as_simple().to_string(),
            "created": get_unix_time()
        });
        let user = self.create_document_auth("auth", username, data).await?;

        if let Value::String(uid) = &user["username"] {
            let jwt = jwt::encode(uid.into(), SECRET)?;
            let token = TokenResponse{ code: 200, token: jwt, status: "OK".into() };
            Ok(token)
        } else {
            Err(Error::Request)
        }
    }
    pub async fn sign_out(&self, jwt: &str) -> Result<()> {
        let claims = jwt::decode(jwt, SECRET)?;
        let data = json!({
            "active": false,
            "last_online": get_unix_time()
        });
        self.merge_auth("auth", &claims.sub, data).await?;
        Ok(())
    }
}