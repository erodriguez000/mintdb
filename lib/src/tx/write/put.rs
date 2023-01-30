use serde_json::{Value, json};
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::tx::txn::Transaction;
use crate::tx::op::Operation;
use crate::prelude::*;

impl<'a> Transaction<'a> {
    pub async fn put_c(&mut self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        let document = self.db.get_one(tb, doc).await;
        match document {
            Ok(mut document) => {
                if let Some(document) = document.as_object_mut() {
                    document.insert(key.into(), value);
                    Ok(json!(document))
                } else {
                    self.ok = true;
                    Err(Error::TxFinished)
                }
            }
            Err(_) => {
                self.ok = true;
                Err(Error::TxFinished)
            }
        }
    }
    pub async fn put(&mut self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get_mut(doc) {
                    Some(document) => {
                        if self.cmt {
                            document.data.insert("modified".into(), json!(crate::util::time::get_unix_time()));
                            document.data.insert(key.into(), value);
                            return Ok(json!(document.data))
                        } else {
                            self.tx.push(Operation::Put { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                            return Ok(json!("OK"))
                        }
                    }
                    None => {
                        if self.cmt {
                            let mut document = Document::new(doc);
                            document.data.insert("created".into(), json!(crate::util::time::get_unix_time()));
                            document.data.insert("id".into(), json!(doc));
                            document.data.insert(key.into(), value);
                            let res = json!(document.data);
                            tbl.documents.insert(doc.into(), document);
                            return Ok(res)
                        } else {
                            self.tx.push(Operation::Put { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                            return Ok(json!("OK"))
                        }
                    }
                }
            }
            None => {
                if self.cmt {
                    let mut document = Document::new(doc);
                    document.data.insert("created".into(), json!(crate::util::time::get_unix_time()));
                    document.data.insert("id".into(), json!(doc));
                    document.data.insert(key.into(), value);
                    let res = json!(document.data);
                    let mut tbl = Table::new(tb);
                    tbl.documents.insert(doc.into(), document);
                    lock.tables.insert(tb.into(), tbl);
                    return Ok(json!(res))
                } else {
                    self.tx.push(Operation::Put { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                    return Ok(json!("OK"))
                }
            }
        }
    }
}