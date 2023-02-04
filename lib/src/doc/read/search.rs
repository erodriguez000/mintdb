use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl Datastore {
    pub(crate) async fn find_auth(&self, tb: &str, data: Value) -> Result<Value> {
        let mut res = vec![];
        let lock = self.collections.try_read().unwrap();
        let tbl = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?;
        for (_, document) in &tbl.documents{
            for (k, val) in data.as_object().ok_or(Error::Request)? {
                if let Some(d) = document.data.get(k) {
                    if d == val {
                        res.push(json!(document.data));
                        break;
                    }
                }
            }
        }
        Ok(json!(res))
    }
    pub(crate) async fn match_auth(&self, tb: &str, data: Value) -> Result<Value> {
        let data = data.as_object().ok_or(Error::Request)?;
        let mut res = vec![];
        let lock = self.collections.try_read().unwrap();
        let tbl = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?;
        for (_, document) in &tbl.documents {
            if data.iter().all(|(k, v)| document.data.contains_key(k) && &document.data[k] == v) {
                res.push(json!(document.data))
            }
        }
        Ok(json!(res))
    }
}