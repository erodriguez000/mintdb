use serde_json::{Value, json};
use crate::tx::op::Tx;
use crate::tx::txn::Transaction;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl<'a> Transaction<'a> {
    pub async fn merge(&mut self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        if self.ok {
            return Err(Error::TxFinished)
        }
        let ts = get_unix_time();
        let document_data = match self.db.get_one(tb, doc).await {
            Ok(val) => {
                let mut document_data = val.as_object()
                .ok_or(Error::Request)?.to_owned();
                let data = data.as_object().ok_or(Error::Request)?.to_owned();
                document_data.insert("modified".into(), json!(&ts));
                for (key, value) in data {
                    document_data.insert(key, value);
                }
                document_data
            },
            Err(_) => {
                let mut data = data.as_object()
                .ok_or(Error::TxFinished)?.to_owned();
                data.insert("created".into(), json!(&ts));
                data.insert("id".into(), doc.into());
                data
            }
        };
        let tx = Tx::new(tb.into(), doc.into(), json!(document_data), &ts);
        let res = json!(document_data);
        self._db.insert(ts, tx);
        Ok(res)
    }
}