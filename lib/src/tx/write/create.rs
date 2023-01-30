use serde_json::{Value, json};
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::tx::op::{Operation, Tx};
use crate::util::time::get_unix_time;
impl<'a> Transaction<'a> {
    pub async fn create_document(&mut self, tb: &str, doc: &str, data: &Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished);
        }
        let data = match data.as_object() {
            Some(val) => val.to_owned(),
            None => {
                self.ok = true;
                return Err(Error::TxFinished)
            }
        };
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get(doc) {
                    Some(_) => {
                        self.ok = true;
                        Err(Error::TxFinished)
                    }
                    None => {
                        if self.cmt {
                            let mut document = Document::new(doc);
                            document.data.insert(f!("created"), json!("now"));
                            document.data.insert("id".into(), json!(doc));
                            for (key, val) in data {
                                document.data.insert(key.into(), json!(val));
                            }
                            let res = json!(document.data);
                            tbl.documents.insert(doc.into(), document);
                            Ok(res)
                        } else {
                            self.tx.push(Operation::CreateDocument { tb: tb.into(), doc: f!("{doc}"), value: json!(data) });
                            Ok(json!(""))
                        }
                    }
                }
            }
            None => {
                if self.cmt {

                    let mut document = Document::new(doc);
                    document.data.insert("created".into(), json!("now"));
                    document.data.insert("id".into(), json!(doc));
                    for (key, val) in data {
                        document.data.insert(f!("{key}"), json!(val));
                    }
                    let mut tbl = Table::new(tb);
                    let res = json!(document.data);
                    tbl.documents.insert(f!("{doc}"), document);
                    lock.tables.insert(f!("{tb}"), tbl);
                    
                    Ok(res)
                } else {
                    self.tx.push(Operation::CreateDocument { tb: tb.into(), doc: doc.into(), value: json!(data) });
                    Ok(json!("Ok"))
                }
            }
        }
    }
    pub async fn create_table(&mut self, tb: &str) -> Result<Value> {
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get(tb) {
            Some(_) => {
                self.ok = true;
                Err(Error::TxFinished)
            }
            None => {
                if self.cmt {
                    let tbl = Table::new(tb);
                    lock.tables.insert(f!("{tb}"), tbl);
                    Ok(json!(f!("created table {tb}")))
                } else {
                    self.tx.push(Operation::CreateTable{ tb: f!("{tb}")} );
                    Ok(json!("Ok"))
                }
            }
        }
    }
    pub async fn create_documentc(&mut self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        match self.db.get_one(tb, doc).await {
            Ok(_) => {
                self.ok = true;
                return Err(Error::DocumentExists(doc.into()));
            }
            Err(_) => ()
        };
        let mut data = data.as_object().ok_or(Error::Request)?.to_owned();
        let ts = get_unix_time();
        data.insert("created".into(), json!(&ts));
        data.insert("id".into(), json!(doc));
        let tx = Tx::new(tb.into(), doc.into(), json!(&data), &ts);
        self._db.insert(ts, tx);
        Ok(json!(data))
    }
}