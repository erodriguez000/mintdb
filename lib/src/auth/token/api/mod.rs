use crate::{prelude::*, kvs::store::Datastore, util::time::get_unix_time};
use serde_json::{json, Value};
use uuid::Uuid;
impl Datastore {
    pub async fn create_api_token(&self, username: &str) -> Result<Value> {
        let mut user = self.get_one_auth("auth", username).await?;
        let data = user.as_object_mut().ok_or(Error::Request)?;
        data.insert(f!("created"), json!(get_unix_time()));
        data.insert(f!("exp"), json!(get_unix_time()));
        let token = Uuid::new_v4().as_simple().to_string();
        self.create_document_auth("token", &token, json!(data)).await?;
        Ok(json!(token))
    }
    pub async fn delete_api_token(&self, token: &str) -> Result<Value> {
        self.delete_document_auth("token", token).await
    }
    pub async fn validate_api_token(&self, token: &str) -> Result<Value> {
        let ctx = self.get_one_auth("token", token).await.ok().ok_or(Error::InvalidAPIToken)?;
        match &ctx["exp"] {
            Value::String(exp) => {
                let ts = get_unix_time();
                if &ts < exp {
                    return Ok(ctx)
                } else {
                    return Err(Error::InvalidAPITokenExp)
                }
            }
            _ => Err(Error::InvalidAPIToken)
        }
    }
}
