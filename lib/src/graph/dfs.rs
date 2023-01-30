use crate::kvs::store::Datastore;
use crate::prelude::*;
use serde_json::Value;
use serde_json::json;

impl Datastore {
    pub fn dfs<'a>(&self, tb: &str, doc: &str, rel: &str, target_doc: &str, visited: &'a mut Vec<Value>) -> Result<Option<Value>> {
        let lock = self.collections.try_read().unwrap();
        let children = match &lock.tables[tb].documents[doc].data[&f!("rel:{rel}")] {
            Value::Array(arr) => arr,
            _ => {
                return Ok(None);
            }
        };
        for child in children {
            let rel_doc = match child.get("doc") {
                Some(Value::String(str)) => str.as_str(),
                _ => continue
            };
            if rel_doc == target_doc {
                return Ok(Some(json!(&lock.tables[tb].documents[doc].data)))
            }
            if visited.contains(&json!(rel_doc)) {
                continue;
            } else {
                visited.push(json!(rel_doc));
            }
            let rel_tb = match child.get("tb") {
                Some(Value::String(str)) => str.as_str(),
                _ => continue
            };

            match self.dfs(rel_tb, rel_doc, &rel, target_doc, visited) {
                Ok(Some(val)) => return Ok(Some(val)),
                _ => continue
            };
        }
        Ok(None)
    }
}