use serde_json::{Value, json};
use crate::tx::op::Tx;
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn put(&mut self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.db.get_one(tb, doc).await {
            Ok(mut document) => {
                if let Some(document) = document.as_object_mut() {
                    let ts = get_unix_time();
                    document.insert("modified".into(), json!(&ts));
                    document.insert(key.into(), value);
                    let tx = Tx::new(tb.into(), doc.into(), json!(document), &ts);
                    self._db.insert(ts, tx);
                    Ok(json!(document))
                } else {
                    self.ok = true;
                    Err(Error::TxFinished)
                }
            }
            Err(e) => {
                self.ok = true;
                Err(e)
            }
        }
    }
}