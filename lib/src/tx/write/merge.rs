use serde_json::{Value, json};
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::tx::op::{Operation, Tx};
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn merge(&mut self, tb: &str, doc: &str, data: &Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let data = match data.as_object() {
            Some(v) => v.to_owned(),
            None => {
                self.ok = true;
                return Err(Error::TxFinished)
            }
        };
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get_mut(doc) {
                    Some(document) => {
                        if self.cmt {   
                            document.data.insert("modified".into(), json!("now"));
                            for (key, val) in data {
                                document.data.insert(key, val);
                            }
                            return Ok(json!(document.data))
                        } else {
                            self.tx.push(Operation::Merge { tb: tb.into(), doc: doc.into(), value: json!(data) });
                            return Ok(json!("OK"))
                        }
                    }
                    None => {
                        if self.cmt {
                            let mut document = Document::new(doc);
                            document.data.insert("created".into(), json!("now"));
                            for (key, val) in data {
                                document.data.insert(key, val);
                            }
                            let res = json!(document.data);
                            tbl.documents.insert(doc.into(), document);
                            return Ok(res)
                        } else {
                            self.tx.push(Operation::Merge { tb: tb.into(), doc: doc.into(), value: json!(data) });
                            return Ok(json!("OK"))
                        }
                    }
                }
            }
            None => {
                if self.cmt {
                    let mut document = Document::new(doc);
                    document.data.insert(f!("created"), json!("now"));
                    for (key, val) in data {
                        document.data.insert(key, val);
                    }
                    let res = json!(document.data);
                    let mut tbl = Table::new(tb);
                    tbl.documents.insert(doc.into(), document);
                    lock.tables.insert(tb.into(), tbl);
                    return Ok(res)
                } else {
                    self.tx.push(Operation::Merge { tb: tb.into(), doc: doc.into(), value: json!(data) });
                    return Ok(json!("OK"))
                }
                
            }
        }
    }
    pub async fn merge_c(&mut self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let ts = get_unix_time();
        let document_data = match self.db.get_one(tb, doc).await {
            Ok(val) => {
                let mut document_data = val.as_object()
                .ok_or(Error::Request)?.to_owned();
                let data = data.as_object().ok_or(Error::Request)?.to_owned();
                document_data.insert("modified".into(), json!(&ts));
                for (key, value) in data {
                    document_data.insert(key, value);
                }
                document_data
            },
            Err(_) => {
                let mut data = data.as_object()
                .ok_or(Error::TxFinished)?.to_owned();
                data.insert("created".into(), json!(&ts));
                data.insert("id".into(), doc.into());
                data
            }
        };
        let tx = Tx::new(tb.into(), doc.into(), json!(document_data), &ts);
        let res = json!(document_data);
        self._db.insert(ts, tx);
        Ok(res)
    }
}