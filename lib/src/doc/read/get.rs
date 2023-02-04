use serde_json::{Value, json};

use crate::kvs::store::Datastore;
use crate::prelude::*;

impl Datastore {
    pub(crate) async fn get_key_auth(&self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let res = lock.tables.get(tb)
        .ok_or(Error::TableNotFound(tb.into()))?
        .documents.get(doc)
        .ok_or(Error::DocumentNotFound(doc.into()))?
        .data.get(key)
        .ok_or(Error::Request)?;
        Ok(json!(res))
    }
    pub(crate) async fn get_one_auth(&self, tb: &str, doc: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let res = lock.tables
            .get(tb)
            .ok_or(Error::TableNotFound(tb.into()))?
            .documents.get(doc)
            .ok_or(Error::DocumentNotFound(doc.into()))?
            .data.clone();
        Ok(json!(res))
    }
    pub(crate) async fn get_many_auth(&self, tb: &str) -> Result<Value> {
        let mut res = vec![];
        let lock = self.collections.try_read().unwrap();
        let tbl = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?;
        for (_, document) in &tbl.documents {
            res.push(document.data.clone());
        }
        Ok(json!(res))
    }
}