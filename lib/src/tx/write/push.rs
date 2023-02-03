use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::tx::op::Tx;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn push(&mut self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let mut document = match self.db.get_one(tb, doc).await {
            Ok(res) => res,
            Err(e) => match e {
                Error::TableNotFound(_) | Error::DocumentNotFound(_) => {
                    json!({
                        "id": doc,
                        "created": get_unix_time(),
                    })
                }
                _ => {
                    self.ok = true;
                    return Err(e)
                }
                
            }
        };
        match document.as_object_mut() {
            Some(document) =>  match document.get_mut(key){
                Some(Value::Array(arr)) => {
                    arr.push(value);
                    let ts = get_unix_time();
                    document.insert("modified".into(), json!(&ts));
                    let tx = Tx::new(tb.into(), doc.into(), json!(document), &ts);
                    self._db.insert(ts, tx);
                    Ok(json!(document))
                }
                None => {
                    let ts = get_unix_time();
                    document.insert("modified".into(), json!(&ts));
                    document.insert(key.into(), json!([value]));
                    let tx = Tx::new(tb.into(), doc.into(), json!(document), &ts);
                    self._db.insert(ts, tx);
                    Ok(json!(document))
                }
                _ => {
                    self.ok = true;
                    Err(Error::KeyContainsInvalidType(key.into()))
                }
            }
            None => {
                self.ok = true;
                Err(Error::TxFinished)
            }
        }
    }
}