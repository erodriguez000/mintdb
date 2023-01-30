use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Document {
    pub id: String,
    pub data: BTreeMap<String, Value>
}
impl Document {
    pub fn new(id: &str) -> Self {
        Document { id: id.to_string(), ..Default::default() }
    }
}
