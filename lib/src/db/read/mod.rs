use serde_json::Value;
use crate::fs::log::write::{log_read_event, log_error};
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl  Datastore {
    /// Returns a document or error if not found
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "person";
    ///     let doc = "person:1";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Lucy"})).await?;
    /// 
    ///     let res = db.get_one(tb, doc).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_one(&self, tb: &str, doc: &str) -> Result<Value> {
        match self.get_one_auth(tb, doc).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}', Document: '{doc}'")).await?;
                Ok(res)
            }   
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Document: '{doc}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Returns all documents in a table or error if not found
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "person";
    ///     let doc = "person:1";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Lucy"})).await?;
    /// 
    ///     let res = db.get_many(tb).await?.as_array().unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_many(&self, tb: &str) -> Result<Value> {
        match self.get_many_auth(tb).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Returns all documents matching any key value pair
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "person";
    ///     let doc = "person:1";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Lucy"})).await?;
    /// 
    ///     let tb = "person";
    ///     let doc = "person:2";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Ricky", "state": "FL"})).await?;
    /// 
    /// 
    ///     let data = json!({"name":"Lucy", "state": "FL"});
    ///     let res = db.find(tb, data).await?.as_array().unwrap();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn find(&self, tb: &str, data: Value) -> Result<Value> {
        match self.find_auth(tb, data).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Returns all documents matching all key value pairs
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "person";
    ///     let doc = "person:1";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Lucy"})).await?;
    /// 
    ///     let tb = "person";
    ///     let doc = "person:2";
    /// 
    ///     db.merge(tb, doc, json!({"name": "Ricky", "state": "FL"})).await?;
    /// 
    /// 
    ///     let data = json!({"name":"Lucy", "state": "FL"});
    ///     let res = db.match_all(tb, data).await?.as_array().unwrap();
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn match_all(&self, tb: &str, data: Value) -> Result<Value> {
        match self.match_auth(tb, data).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Returns all documents matching comparison
    /// Comparison operators: "==", "!=", ">=", ">", "<=", "<", "contains" (case sensitive), "icontains" (case insensitive)
    /// 
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// use serde_json::json;
    /// 
    /// #[tokio::main]
    /// async fn main() ->  Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     let tb = "car";
    ///     let doc = "car:1";
    /// 
    ///     db.merge(tb, doc, json!({"make": "Mercedes-Benz", "year": 2024})).await?;
    /// 
    ///     let tb = "car";
    ///     let doc = "car:2";
    /// 
    ///     db.merge(tb, doc, json!({"make": "Suzuki", "year": 1995})).await?;
    /// 
    /// 
    ///     let lhs = "make";
    ///     let op = "icontains";
    ///     let rhs = json!("benz");
    ///     let res = db.compare(tb, lhs, op, &rhs).await?.as_array().unwrap();
    /// 
    ///     let lhs = "year";
    ///     let op = ">=";
    ///     let rhs = json!(2024);
    ///     let res = db.compare(tb, lhs, op, &rhs).await?.as_array().unwrap();
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn compare(&self, tb: &str, lhs: &str, op: &str, rhs: &Value) -> Result<Value> {
        match self.compare_auth(tb, lhs, op, rhs).await {
            Ok(res) => {
                log_read_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("SELECT >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
}