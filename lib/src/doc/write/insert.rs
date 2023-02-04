use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl Datastore {
    pub(crate) async fn insert_auth(&self, tb:&str, doc: &str, key: &str, value: Value) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            if let Some(document) = tbl.documents.get_mut(doc) {
                if document.data.contains_key(key) {
                    return Err(Error::KeyExists(key.into(), doc.into()))
                }
                document.data.insert("modified".into(), json!(get_unix_time()));
                document.data.insert(key.into(), value);
                Ok(json!(document.data))
            } else {
                let mut document = Document::new(doc);
                document.data.insert("created".into(), json!(get_unix_time()));
                document.data.insert("id".into(), json!(doc));
                document.data.insert(key.into(), value);
                let res = json!(document.data);
                tbl.documents.insert(doc.into(), document);
                Ok(res)
            }
        } else {
            let mut document = Document::new(doc);
            document.data.insert("created".into(), json!(get_unix_time()));
            document.data.insert("id".into(), json!(doc));
            document.data.insert(key.into(), value);
            let mut tbl = Table::new(tb);
            let res = json!(document.data);
            tbl.documents.insert(doc.to_string(), document);
            lock.tables.insert(tb.into(), tbl);
            Ok(res)
        }
    }
}