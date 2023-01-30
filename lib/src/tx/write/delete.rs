use serde_json::{Value, json};

use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::tx::op::{Operation, Tx};
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn delete_key(&mut self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let mut lock = self.db.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            if let Some(document) = tbl.documents.get_mut(doc) {
                if let Some(_) = document.data.get(key) {
                    if self.cmt {
                        document.data.insert(f!("modified"), json!("Now"));
                        document.data.remove(key);
                        return Ok(json!(document.data))
                    } else {
                        self.tx.push(Operation::DeleteKey { tb: tb.into(), doc: doc.into(), key: key.into() });
                        return Ok(json!("OK"))
                    }
                }
            }
        }
        self.ok = true;
        Err(Error::TxFinished)
    }
    pub async fn delete_document(&mut self, tb: &str, doc: &str) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let mut lock = self.db.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            if let Some(_) = tbl.documents.get(doc) {
                if self.cmt {
                    let res = tbl.documents.remove(doc).ok_or(Error::Request)?;
                    return Ok(json!(res.data))
                } else {
                    self.tx.push(Operation::DeleteDocument{ tb: tb.into(), doc: doc.into()});
                    return Ok(json!("OK"));
                }
            }
        }
        self.ok = true;
        Err(Error::TxFinished)
    }
    pub async fn delete_tb(&mut self, tb: &str) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let mut lock = self.db.collections.try_write().unwrap();
        if let Some(_) = lock.tables.get(tb) {
            if self.cmt {
                let res = lock.tables.remove(tb).ok_or(Error::Request)?;
                return Ok(json!(res.documents))
            } else {
                self.tx.push(Operation::DeleteTable { tb: tb.into() });
                return Ok(json!("OK"))
            }
        }
        self.ok = true;
        Err(Error::TxFinished)
    }
    pub async fn delete_keyc(&mut self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        match self.db.get_key_auth(tb, doc, key).await {
            Ok(_) => (),
            Err(_) => {
                self.ok = true;
                return Err(Error::KeyNotFound(key.into(), doc.into()))
            }
        };
        let mut data = match self.db.get_one(tb, doc).await {
            Ok(val) => val.as_object().ok_or(Error::Request)?.to_owned(),
            Err(_) => {
                return Err(Error::DocumentNotFound(doc.into()));
            }
        };
        let ts = get_unix_time();
        data.insert("modified".into(), json!(ts));
        data.remove(key);
        let tx = Tx::new(tb.into(), doc.into(), json!(data), &ts);
        let res = json!(data);
        self._db.insert(ts, tx);
        Ok(res)
    }
}