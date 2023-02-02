use serde_json::Value;
use crate::fs::log::write::{log_read_event, log_error};
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl  Datastore {
    pub async fn get_one(&self, tb: &str, doc: &str) -> Result<Value> {
        match self.get_one_auth(tb, doc).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}', Document: '{doc}'")).await?;
                Ok(res)
            }   
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Document: '{doc}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn get_many(&self, tb: &str) -> Result<Value> {
        match self.get_many_auth(tb).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn find(&self, tb: &str, data: Value) -> Result<Value> {
        match self.find_auth(tb, data).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn match_all(&self, tb: &str, data: Value) -> Result<Value> {
        match self.match_auth(tb, data).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn compare(&self, tb: &str, lhs: &str, op: &str, rhs: &Value) -> Result<Value> {
        match self.compare_auth(tb, lhs, op, rhs).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
}