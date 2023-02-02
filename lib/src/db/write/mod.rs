use serde_json::{Value, json};
use crate::fs::ds::write::{commit_document, delete_document, delete_table};
use crate::fs::log::write::{log_create_event, log_error, log_merge_event, log_delete_event};
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl  Datastore {
    pub async fn create_document(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.create_document_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_create_event(&f!("Table: '{tb}', Document: '{doc}', Value: '{}'", json!(res))).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("CREATE FAILED >> Table: '{tb}', Document: '{doc}', Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn create_table(&self, tb: &str) -> Result<Value> {
        match self.create_table_auth(tb).await {
            Ok(res) => {
                log_create_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("CREATE >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn merge(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.merge_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("Table: '{tb}', Document: '{doc}', Value: '{}'", json!(res))).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("MERGE FAILED >> Table: {tb}, Document: '{doc}', Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn insert(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.insert_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn push(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.push_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("PUSH >> Table: '{tb}', Document: '{doc}', key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("PUSH >> Table: {tb}, Document: '{doc}', Key: '{key}' Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn put(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.put_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("PUT >> Table: '{tb}', Document: '{doc}', key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("MERGE PUT FAILED >> Table: {tb}, Document: '{doc}', Key: '{key}' Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn delete_key(&self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        match self.delete_key_auth(tb, doc, key).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_delete_event(&f!("Table: '{tb}', Document: '{doc}', Key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Document: '{doc}', Key: '{key}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn delete_document(&self, tb: &str, doc: &str) -> Result<Value> {
        match self.delete_document_auth(tb, doc).await {
            Ok(res) => {
                delete_document(tb, doc).await?;
                log_delete_event(&f!("Table: '{tb}', Document: '{doc}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Document: '{doc}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn delete_key_from_table(&self, tb: &str, key: &str) -> Result<Value> {
        match self.delete_key_from_table_auth(tb, key).await {
            Ok(res) => {
                log_delete_event(&f!("REMOVED KEY >> Table: '{tb}', Key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Key: '{key}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn delete_table(&self, tb: &str) -> Result<Value> {
        match self.delete_table_auth(tb).await {
            Ok(res) => {
                delete_table(tb).await?;
                log_delete_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
}