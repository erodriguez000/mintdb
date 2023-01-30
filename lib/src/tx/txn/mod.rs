use std::collections::HashMap;
use std::sync::{Arc};
use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::prelude::*;

use super::op::{Operation, Tx};


impl Datastore {
    pub async fn transaction(& self) -> Result<Transaction> {
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
    pub async fn commit(&mut self) -> Result<Value> {
        if self.ok {
            return Err(Error::Request)
        }
        self.cmt = true;
        let mut res = vec![];
        for op in self.tx.clone() {
            match op {
                Operation::Credit { tb, doc, key, rhs } => {
                    let v = self.credit(&tb, &doc, &key, rhs).await?;
                    res.push(v);
                }
                Operation::Debit { tb, doc, key, rhs } => {
                    let v = self.debit(&tb, &doc, &key, rhs).await?;
                    res.push(v);
                }
                Operation::CreateTable { tb } => {
                    let v = self.create_table(&tb).await?;
                    res.push(v);
                }
                Operation::CreateDocument { tb, doc, value } => {
                    let v = self.create_document(&tb, &doc, &value).await?;
                    res.push(v)
                }
                Operation::Update { tb, doc, key, value } => {
                    let v = self.update(&tb, &doc, &key, &value).await?;
                    res.push(v);
                }
                Operation::Insert { tb, doc, key, value } => {
                    let v = self.insert(&tb, &doc, &key, &value).await?;
                    res.push(v);
                }
                Operation::Merge { tb, doc, value } => {
                    let v = self.merge(&tb, &doc, &value).await?;
                    res.push(v);
                }
                Operation::Push { tb, doc, key, value } => {
                    let v = self.push(&tb, &doc, &key, value).await?;
                    res.push(v)
                }
                Operation::Put { tb, doc, key, value } => {
                    let v = self.put(&tb, &doc, &key, value).await?;
                    res.push(v);
                }
                Operation::DeleteKey { tb, doc, key } => {
                    let v = self.delete_key(&tb, &doc, &key).await?;
                    res.push(v);
                }
                Operation::DeleteDocument { tb, doc } => {
                    let v = self.delete_document(&tb, &doc).await?;
                    res.push(v);
                }
                Operation::DeleteTable { tb } => {
                    let v = self.delete_tb(&tb).await?;
                    res.push(v);
                }
            }
        }
        
        Ok(json!(res))
    }
    pub async fn commit_c(&self) -> Result<()> {
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
        Ok(())
    }
}