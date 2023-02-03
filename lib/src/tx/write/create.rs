use serde_json::{Value, json};
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::tx::op::Tx;
use crate::util::time::get_unix_time;
impl<'a> Transaction<'a> {
    pub async fn create_document(&mut self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        match self.db.get_one(tb, doc).await {
            Ok(_) => {
                self.ok = true;
                return Err(Error::DocumentExists(doc.into()));
            }
            Err(_) => ()
        };
        let mut data = data.as_object().ok_or(Error::Request)?.to_owned();
        let ts = get_unix_time();
        data.insert("created".into(), json!(&ts));
        data.insert("id".into(), json!(doc));
        let tx = Tx::new(tb.into(), doc.into(), json!(&data), &ts);
        self._db.insert(ts, tx);
        Ok(json!(data))
    }
}