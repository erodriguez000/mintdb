use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::tx::op::Tx;
use crate::util::time::get_unix_time;
impl<'a> Transaction<'a> {
    pub async fn debit(&mut self, tb: &str, doc: &str, key: &str, amt: f64) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished);
        }
        let mut document = match self.db.get_one(tb, doc).await {
            Ok(val) => match val.as_object(){
                Some(obj) => obj.to_owned(),
                None => {
                    self.ok = true;
                    return Err(Error::Request);
                }
            },
            Err(e) => {
                self.ok = true;
                return Err(e)
            }
        };
        if let Some(balance) = document.get_mut(key) {
            match balance {
                Value::Number(balance) => {
                    if let Some(bal) = balance.as_f64() {
                        if bal - amt < 0.00 {
                            self.ok = true;
                            return Err(Error::TxFinished)
                        }
                        let val = json!(bal - amt);
                        let ts = get_unix_time();
                        document.insert("modified".into(), json!(&ts));
                        document.insert(key.into(), val);
                        let res = json!(document);
                        let tx = Tx::new(tb.into(), doc.into(), document.into(), &ts);
                        self._db.insert(ts, tx);
                        Ok(res)

                    } else {
                        self.ok = true;
                        return Err(Error::KeyContainsInvalidType(key.into()));
                    }
                }
                _ => {
                    self.ok = true;
                    return Err(Error::KeyContainsInvalidType(key.into()));
                }
            }
        } else {
            self.ok = true;
            Err(Error::KeyNotFound(key.into(), doc.into()))
        }
    }
}