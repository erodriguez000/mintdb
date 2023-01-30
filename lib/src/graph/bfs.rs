use std::collections::VecDeque;

use serde_json::{Value, json};

use crate::kvs::store::Datastore;
use crate::prelude::*;

impl Datastore {
    pub async fn bfs<'a>(&self, tb: &str, doc: &str, rel: &str, target_doc: &str, visited: &'a mut Vec<Value>) -> Result<Option<Value>> {
        let mut queue = VecDeque::new();
        let rel = f!("rel:{rel}");
        let lock = self.collections.try_read().unwrap();

        queue.push_back((tb, doc));

        while !queue.is_empty() {
            let (current_tb, current_doc) = queue.pop_front().unwrap();
            if current_doc == target_doc {
                return Ok(Some(json!(lock.tables[current_tb].documents[current_doc].data.clone())))
            }
            let children = match &lock.tables[current_tb].documents[current_doc].data[&rel] {
                Value::Array(a) => {
                    a
                }
                _ => continue
            };

            for child in children {
                let rel_doc = match &child["doc"] {
                    Value::String(str) => str.as_str(),
                    _ => continue
                };
                let rel_tb = match &child["tb"] {
                    Value::String(str) => str.as_str(),
                    _ => continue
                };

                if rel_doc == target_doc {
                    return Ok(Some(json!(lock.tables[current_tb].documents[current_doc].data.clone())))
                }
                if visited.contains(&json!(rel_doc)) {
                    continue;
                } else {
                    visited.push(json!(rel_doc));
                }
                queue.push_back((rel_tb, rel_doc));
            }
        }
        Ok(None)
    }
}