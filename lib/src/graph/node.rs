use serde_json::json;

use crate::kvs::store::Datastore;
use crate::prelude::*;
use crate::util::time::get_unix_time;

impl Datastore {
    pub async fn add_node(&self, tb: &str, doc: &str, ref_tb: &str, ref_doc: &str, rel: &str) -> Result<()> {
        let mut lock = self.collections.write().unwrap();
        match lock.tables.get_mut(tb) {
            Some(tbl) => {
                match tbl.documents.get_mut(doc) {
                    Some(document) => {
                        let node = f!("rel:{rel}");
                        match document.data.get_mut(&node) {
                            Some(arr) => {
                                // Node key does exist
                                match arr.as_array_mut() {
                                    Some(array) => {
                                        let value = json!({
                                            "tb": ref_tb,
                                            "doc": ref_doc,
                                        });
                                        array.push(value);
                                        document.data.insert("modified".into(), json!(get_unix_time()));
                                    }
                                    None => {
                                        return Err(Error::Request);
                                    }
                                }
                            }
                            None => {
                                // Node key does not exist
                                let value = vec![json!({
                                    "tb": ref_tb,
                                    "doc": ref_doc,
                                })];
                                document.data.insert("modified".into(), json!(get_unix_time()));
                                document.data.insert(node, json!(value));
                            }
                        }
                    }
                    None => {
                        // Document does not exist
                        return Err(Error::Request);
                    }
                }
            }
            None => {
                // Table does not exist
                return Err(Error::Request);
            }
        };
        Ok(())
    }
}