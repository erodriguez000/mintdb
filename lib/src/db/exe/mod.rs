use serde::Deserialize;
use serde_json::{Value, json};

use crate::kvs::store::Datastore;
use crate::prelude::*;
#[derive(Deserialize, Debug)]
pub struct SQL {
    pub stmt: String,
    pub tb: String,
    pub doc: String,
    pub data: Value,
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}
impl Datastore {
    /// executes a SQL statement
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// use mintdb::db::exe::SQL;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "car";
    ///     let doc = "car:1";
    ///     let data = json!({"make": "Mercedes-Benz"});
    ///     let sql = SQL{
    ///         stmt: "MERGE".into(),
    ///         tb: tb.into(),
    ///         doc: doc.into(),
    ///         data: data,
    ///         topic: "".into(),
    ///         user_id: Some(1),
    ///         message: "".into(),    
    ///     };
    ///     db.execute(&sql).await?;
    /// 
    ///     let sql = SQL{
    ///         stmt: "SELECT".into(),
    ///         tb: "car".into(),
    ///         doc: "*".into(),
    ///         data: json!({}),
    ///         topic: "".into(),
    ///         user_id: Some(1),
    ///         message: "".into(),    
    ///     };
    ///     let res = db.execute(&sql).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&self, sql: &SQL) -> Result<Value> {
        match sql.stmt.as_str() {
            "INFO" => {
                match self.get_tb_keys_auth().await {
                    Ok(res) => Ok(json!(&res)),
                    Err(e) => Ok(json!({"error": e.to_string()}))
                }
            }
            "ADD" => {
                match self.create_table(&sql.tb).await {
                    Ok(res) => Ok(json!(&res)),
                    Err(e) => Ok(json!({"error": e.to_string()}))
                }
            }
            "SELECT" => {
                match sql.doc.as_str() {
                    "*" => {
                        match self.get_many(&sql.tb).await {
                            Ok(res) => return Ok(json!(&res)),
                            Err(e) => return Ok(json!({"error": e.to_string()})),
                        }
                    }
                    _ => {
                        match self.get_one(&sql.tb, &sql.doc).await {
                            Ok(res) => return Ok(json!(&res)),
                            Err(e) => return Ok(json!({"error": e.to_string()})),
                        }
                    }
                }
            }
            "MERGE" => self.merge(&sql.tb, &sql.doc, json!(sql.data)).await,
            "CREATE" => self.create_document(&sql.tb, &sql.doc, json!(sql.data)).await,
            "DELETE" => {
                if let Value::String(key) = &sql.data["key"] {
                    if &sql.doc == "*" {
                        return self.delete_key_from_table(&sql.tb, key).await;
                    } else {
                        return self.delete_key(&sql.tb, &sql.doc, key).await;
                    }
                }
                self.delete_document(&sql.tb, &sql.doc).await
    
            }
            "FIND" => self.find(&sql.tb, json!(sql.data)).await,
            "MATCH" => self.match_all(&sql.tb, json!(sql.data)).await,
            "COMPARE" => {
                let lhs = match &sql.data["lhs"] {
                    Value::String(s) => s,
                    _ => return Err(Error::RequestMissingKey("lhs".into()))
                };
                let op = match &sql.data["op"] {
                    Value::String(s) => s,
                    _ => return Err(Error::RequestMissingKey("op".into()))
                };
                let rhs = match &sql.data["rhs"] {
                    Value::Null => return Err(Error::RequestMissingKey("rhs".into())),
                    val => val,
                };
                self.compare(&sql.tb, lhs, op, rhs).await
            }
            "PUSH" => {
                let key = match sql.data.get("key") {
                    Some(k) => match k.as_str() {
                        Some(t) => t,
                        None => return Err(Error::RequestMissingKey("key".into()))
                    }
                    None => return Err(Error::RequestMissingKey("key".into()))
                };
                let value = match sql.data.get("value") {
                    Some(v) => v,
                    None => return Err(Error::RequestMissingKey("value".into()))
                };
                self.push(&sql.tb, &sql.doc, key, value.clone()).await
            }
            "PUT" => {
                let key = match sql.data.get("key") {
                    Some(k) => match k.as_str(){
                        Some(t) => t,
                        None => return Err(Error::RequestMissingKey("key".into()))
                    }
                    None => return Err(Error::RequestMissingKey("key".into()))
                };
                let value = match sql.data.get("value") {
                    Some(v) => v,
                    None => return Ok(json!({"error": "Missing 'value' field in request body"}))
                };
                self.put(&sql.tb, &sql.doc, key, value.clone()).await
            }
            "REL" => {
                let ref_tb = match sql.data.get("ref_tb") {
                    Some(k) => match k.as_str(){
                        Some(t) => t,
                        None => return Err(Error::RequestMissingKey("ref_tb".into()))
                    }
                    None => return Err(Error::RequestMissingKey("ref_tb".into()))
                };
                let ref_doc = match sql.data.get("ref_doc") {
                    Some(k) => match k.as_str(){
                        Some(t) => t,
                        None => return Err(Error::RequestMissingKey("ref_doc".into()))
                    }
                    None => return Err(Error::RequestMissingKey("ref_doc".into()))
                };
                let rel = match sql.data.get("rel") {
                    Some(k) => match k.as_str(){
                        Some(t) => t,
                        None => return Err(Error::RequestMissingKey("rel".into()))
                    }
                    None => return Err(Error::RequestMissingKey("rel".into()))
                };
                match self.add_node(&sql.tb, &sql.doc, ref_tb, ref_doc, rel).await {
                    Ok(_) => {
                        let res = format!("Relationship {} added from {} to {}", rel, sql.doc, ref_doc);
                        return Ok(json!({"Ok": res}))
                    },
                    Err(e) => Ok(json!({"error": e.to_string()}))
                }
            }
            "DFS" => {
                let rel = match &sql.data["rel"] {
                    Value::String(k) => k.as_str(),
                    _ => return Err(Error::RequestMissingKey("rel".into()))
                };
                let target_doc = match &sql.data["target_doc"] {
                    Value::String(k) => k.as_str(),
                    _ => return Err(Error::RequestMissingKey("target_doc".into()))
                };
                let mut visited = vec![];
                match self.dfs(&sql.tb, &sql.doc, rel, target_doc, &mut visited) {
                    Ok(res) => match res {
                        Some(val) => Ok(json!(val)),
                        None => Ok(json!({ "error": "Not found" }))
                    }
                    Err(e) => Ok(json!({ "error": e.to_string() }))
                }
            }
            "BFS" => {
                let rel = match &sql.data["rel"] {
                    Value::String(k) => k.as_str(),
                    _ => return Err(Error::RequestMissingKey("rel".into()))
                };
                let target_doc = match &sql.data["target_doc"] {
                    Value::String(k) => k.as_str(),
                    _ => return Err(Error::RequestMissingKey("target_doc".into()))
                };
                let mut visited = vec![];
                match self.bfs(&sql.tb, &sql.doc, rel, target_doc, &mut visited).await {
                    Ok(res) => match res {
                        Some(val) => Ok(json!(val)),
                        None => Ok(json!({ "error": "Not found" }))
                    }
                    Err(e) => Ok(json!({ "error": e.to_string() }))
                }
            }
            _ => {
                let res = json!({"error": "ERROR: STMT NOT RECOGNIZED"});
                Ok(json!(&res))
            }
        }
    }
}