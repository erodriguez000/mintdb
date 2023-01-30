use serde_json::{Value, json};
use crate::kvs::table::Table;
use crate::kvs::document::Document;
use crate::tx::txn::Transaction;
use crate::tx::op::Operation;
use crate::prelude::*;

impl<'a> Transaction<'a> {
    pub async fn push(&mut self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let mut lock = self.db.collections.try_write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get_mut(doc) {
                    Some(document) => {
                       match document.data.get_mut(key) {
                        Some(val) => {
                            match val.as_array_mut() {
                                Some(arr) => {
                                    if self.cmt {
                                        arr.push(value);
                                        return Ok(json!(document.data))
                                    } else {
                                        self.tx.push(Operation::Push { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                                        return Ok(json!("OK"))
                                    }
                                }
                                None => {
                                    self.ok = true;
                                    return Err(Error::TxFinished)                                
                                }   
                            }
                        }
                        None => {
                            if self.cmt {
                                let val = vec![value];
                                document.data.insert("modified".into(), json!("now"));
                                document.data.insert(key.into(), json!(val));
                                return Ok(json!(document.data))
                            } else {
                                self.tx.push(Operation::Push { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                                return Ok(json!("OK"))
                            }
                        }
                       }
                    }
                    None => {
                        if self.cmt {
                            let val = vec![value];
                            let mut document = Document::new(doc);
                            document.data.insert("created".into(), json!("now"));
                            document.data.insert("id".into(), json!(doc));
                            document.data.insert(key.into(), json!(val));
                            let res = json!(document.data);
                            tbl.documents.insert(doc.into(), document);
                            println!("returning after pushing");
                            return Ok(res)
                        } else {
                            self.tx.push(Operation::Push { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                            return Ok(json!("OK"))
                        }
                    }
                }
            }
            None => {
                if self.cmt {
                    let mut document = Document::new(doc);
                    document.data.insert("created".into(), json!("now"));
                    document.data.insert("id".into(), json!(doc));
                    document.data.insert(key.into(), json!([value]));
                    let res = json!(document.data);
                    let mut tbl = Table::new(tb);
                    tbl.documents.insert(doc.into(), document);
                    lock.tables.insert(tb.into(), tbl);
                    return Ok(json!(res))
                } else {
                    self.tx.push(Operation::Push { tb: tb.into(), doc: doc.into(), key: key.into(), value: value });
                    return Ok(json!("OK"))
                }
            }
        }
    }
}