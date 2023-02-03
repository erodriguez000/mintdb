use std::collections::HashMap;
use std::sync::Arc;
use serde_json::json;
use crate::kvs::store::Datastore;
use crate::prelude::*;

use super::op::{Operation, Tx};


impl Datastore {
    pub async fn transaction(&self) -> Result<Transaction> {
        let ok = true;
        let cmt = false;
        let _db = HashMap::new();
        let db = Arc::new(self);
        let tx = vec![];
        Ok(Transaction {
            ok, cmt, _db, db, tx
        })
    }
}
pub struct Transaction<'a> {
    pub(crate) ok: bool,
    pub(crate) cmt: bool,
    pub (crate) _db: HashMap<String, Tx>,
    pub (crate) db: Arc<&'a Datastore>,
    pub (crate) tx: Vec<Operation>,
}

impl<'a> Transaction<'a> {
    pub fn begin(&mut self) {
        self.ok = false;
        self.cmt = false;
        self.tx.clear();
        self._db.clear();
    }
    pub async fn commit(&mut self) -> Result<()> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        for(ts, tx) in self._db.iter() {
            match self.db.get_one(&tx.tb, &tx.doc).await {
                Ok(val) => {
                    let val = val.as_object().ok_or(Error::Request)?;
                    if val.contains_key("modified") {
                        if ts >= &val["modified"].as_str().ok_or(Error::Request)?.to_string() {
                            continue;
                        } else {
                            return Err(Error::TxFinished)
                        }
                    } else if val.contains_key("created") {
                        if ts >= &val["created"].as_str().ok_or(Error::Request)?.to_string() {
                            continue;
                        } else {
                            return Err(Error::TxFinished)
                        }
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
        for(_, tx) in self._db.iter() {
            self.db.put_document(&tx.tb, &tx.doc, json!(tx.data)).await?;
        }
        self.ok = true;
        Ok(())
    }
}