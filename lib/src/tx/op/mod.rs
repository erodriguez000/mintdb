use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub enum Operation {
    Update{tb: String, doc: String, key: String, value: Value},
    Insert{tb: String, doc: String, key: String, value: Value},
    DeleteKey {tb: String, doc: String, key: String},
    DeleteDocument{tb: String, doc: String},
    DeleteTable{tb: String},
    Debit{tb: String, doc: String, key: String, rhs: f64},
    Credit{tb: String, doc: String, key: String, rhs: f64},
    CreateDocument{tb: String, doc: String, value: Value},
    CreateTable{tb: String},
    Merge{tb: String, doc: String, value: Value},
    Push{tb: String, doc: String, key: String, value: Value},
    Put{tb: String, doc: String, key: String, value: Value},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Tx {
    pub modified: String,
    pub data: Value,
    pub tb: String,
    pub doc: String,
}
impl Tx {
    pub fn new(tb: String, doc: String, data: Value, modified: &str) -> Tx {
        Tx { tb, doc, data, modified: modified.into() }
    }
}