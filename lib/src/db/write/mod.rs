use serde_json::{Value, json};
use crate::fs::ds::write::{commit_document, delete_document, delete_table};
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl  Datastore {
    pub async fn create_document(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.create_document_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn create_table(&self, tb: &str) -> Result<Value> {
        match self.create_table_auth(tb).await {
            Ok(res) => {
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn merge(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.merge_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
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
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn put(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.put_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn delete_key(&self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        match self.delete_key_auth(tb, doc, key).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn delete_document(&self, tb: &str, doc: &str) -> Result<Value> {
        match self.delete_document_auth(tb, doc).await {
            Ok(res) => {
                delete_document(tb, doc).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn delete_table(&self, tb: &str) -> Result<Value> {
        match self.delete_table_auth(tb).await {
            Ok(res) => {
                delete_table(tb).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}