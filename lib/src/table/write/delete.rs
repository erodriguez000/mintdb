use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::prelude::*;
use crate::util::time::get_unix_time;
impl Datastore {
    pub(crate) async fn delete_table_auth(&self, tb: &str) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        lock.tables.remove(tb)
        .ok_or(Error::TableNotFound(tb.into()))?;
        Ok(json!(f!("Table '{tb}' deleted")))
    }
    pub(crate) async fn delete_key_from_table_auth(&self, tb: &str, key: &str) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            for (_, document) in tbl.documents.iter_mut() {
                document.data.insert("modified".into(), json!(get_unix_time()));
                document.data.remove(key);
            }
            Ok(json!(f!("Key '{key}' deleted from Table '{tb}'")))
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
}