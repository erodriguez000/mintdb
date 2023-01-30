use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::tx::op::Operation;
use crate::prelude::*;

impl<'a> Transaction<'a> {
    pub async fn insert(&mut self, tb: &str, doc: &str, key: &str, value: &Value) -> Result<Value> {
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get_mut(doc) {
                    Some(document) => {
                        match document.data.get(key) {
                            Some(_) => {
                                self.ok = true;
                                Err(Error::TxFinished)
                            }
                            None => {
                                if self.cmt {
                                    document.data.insert(f!("{key}"), json!(value));
                                    Ok(json!(document.data))
                                } else {
                                    self.tx.push(Operation::Insert{tb: tb.into(), doc: doc.into(), key: key.into(), value: json!(value)});
                                    Ok(json!("OK"))
                                }
                            }
                        }
                    }
                    None => {
                        self.ok = true;
                        Err(Error::TxFinished)
                    }
                }
            }
            None => {
                self.ok = true;
                Err(Error::TxFinished)
            }
        }
    }
}