use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl Datastore {
    pub(crate) async fn delete_key_auth(&self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        let document = lock.tables
        .get_mut(tb)
        .ok_or(Error::TableNotFound(tb.into()))?
        .documents.get_mut(doc)
        .ok_or(Error::DocumentNotFound(doc.into()))?;
        document.data.remove(key)
        .ok_or(Error::KeyNotFound(key.into(), doc.into()))?;
        document.data.insert("modified".into(), json!(get_unix_time()));
        
        Ok(json!(document.data))
    }
    pub(crate) async fn delete_document_auth(&self, tb: &str, doc: &str) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        let document = lock.tables.get_mut(tb)
        .ok_or(Error::TableNotFound(tb.into()))?
        .documents.remove(doc)
        .ok_or(Error::DocumentNotFound(doc.into()))?;
        Ok(json!(document.data))
    }
}