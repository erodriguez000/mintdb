use serde_json::{Value, json};

use crate::kvs::store::Datastore;
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl Datastore {
    pub(crate) async fn create_document_auth(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        let data = data.as_object().ok_or(Error::Request)?.to_owned();
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            if let Some(_) = tbl.documents.get_mut(doc) {
                Err(Error::DocumentExists(doc.into()))
            } else {
                let mut document = Document::new(doc);
                document.data.insert("created".into(), json!(get_unix_time()));
                document.data.insert("id".into(), json!(doc));
                for (key, value) in data {
                    document.data.insert(key, value);
                }
                let res = json!(document.data);
                tbl.documents.insert(doc.into(), document);
                Ok(res)
            }
        } else {
            let mut document = Document::new(doc);
            document.data.insert("created".into(), json!(get_unix_time()));
            document.data.insert("id".into(), json!(doc));
            for (key, value) in data {
                document.data.insert(key, value);
            }
            let mut tbl = Table::new(tb);
            let res = json!(document.data);
            tbl.documents.insert(doc.to_string(), document);
            lock.tables.insert(tb.into(), tbl);
            Ok(res)
        }
    }
}