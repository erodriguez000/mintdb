use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::tx::op::Tx;
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn delete_key(&mut self, tb: &str, doc: &str, key: &str) -> Result<Value> {
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