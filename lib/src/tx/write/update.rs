use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::tx::op::Operation;
use crate::prelude::*;

impl<'a> Transaction<'a> {
    pub async fn update(&mut self, tb: &str, doc: &str, key: &str, value: &Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished);
        }
        let mut lock = self.db.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            if let Some(document) = tbl.documents.get_mut(doc) {
                if let Some(_) = document.data.get(key) {
                    if self.cmt {
                        document.data.insert(f!("modified"), json!(crate::util::time::get_unix_time()));
                        document.data.insert(f!("{key}"), json!(value));
                        return Ok(json!(document.data))

                    } else {
                        self.tx.push(Operation::Update { tb: tb.into(), doc: doc.into(), key: key.into(), value: json!(value) });
                        return Ok(json!("OK"))
                    }
                }
            }
        }
        self.ok = true;
        Err(Error::TxFinished)
    }
}