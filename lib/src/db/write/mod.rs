use serde_json::{Value, json};
use crate::fs::ds::write::{commit_document, delete_document, delete_table};
use crate::fs::log::write::{log_create_event, log_error, log_merge_event, log_delete_event};
use crate::kvs::store::Datastore;
use crate::prelude::*;
impl  Datastore {
    /// Creates a document, returns new document or error if document exists
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
    ///     match db.create_document(tb, doc, json!({"name": "Lucy"})).await {
    ///         Ok(res) => {
    ///             println!("New document created");
    ///         }
    ///         Err(e) => {
    ///             println!("Document already exists");
    ///         }
    ///     };
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_document(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.create_document_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_create_event(&f!("Table: '{tb}', Document: '{doc}', Value: '{}'", json!(res))).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("CREATE FAILED >> Table: '{tb}', Document: '{doc}', Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Creates a table, returns string confirmation or error if table exists
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
    /// 
    ///     match db.create_table(tb).await {
    ///         Ok(res) => {
    ///             println!("New document created");
    ///         }
    ///         Err(e) => {
    ///             println!("Document already exists");
    ///         }
    ///     };
    ///
    ///     Ok(())
    /// }
    pub async fn create_table(&self, tb: &str) -> Result<Value> {
        match self.create_table_auth(tb).await {
            Ok(res) => {
                log_create_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("CREATE >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Updates or creates a document, returns updated document or error if document exists
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
    ///     db.merge(tb, doc, json!({"email": "lucy@gmail.com"})).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn merge(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.merge_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("Table: '{tb}', Document: '{doc}', Value: '{}'", json!(res))).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("MERGE FAILED >> Table: {tb}, Document: '{doc}', Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    pub async fn insert(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.insert_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    /// Updates or creates a document and pushes the value to an array in the key, 
    /// returns updated document or error if document contains the key and it is not an array.
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
    ///     let key = "friends";
    ///     let value = json!("person:2");
    /// 
    ///     db.push(tb, doc, key, value).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn push(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.push_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("PUSH >> Table: '{tb}', Document: '{doc}', key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("PUSH >> Table: {tb}, Document: '{doc}', Key: '{key}' Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Updates or creates a document and inserts the value into the key, 
    /// returns updated document or error.
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
    ///     let key = "city";
    ///     let value = json!("Miami");
    /// 
    ///     db.put(tb, doc, key, value).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn put(&self, tb: &str, doc: &str, key: &str, value: Value) -> Result<Value> {
        match self.put_auth(tb, doc, key, value).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("PUT >> Table: '{tb}', Document: '{doc}', key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("MERGE PUT FAILED >> Table: {tb}, Document: '{doc}', Key: '{key}' Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Inserts a document into a table, overwriting the previous document if it exists, 
    /// returns updated document or error if document contains the key and it is not an array.
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
    ///     let data = json!({"name": "Lucy"});
    /// 
    ///     db.put_document(tb, doc, data).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn put_document(&self, tb: &str, doc: &str, data: Value) -> Result<Value> {
        match self.put_document_auth(tb, doc, data).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_merge_event(&f!("PUT DOCUMENT >> Table: '{tb}', Document: '{doc}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("MERGE PUT FAILED >> Table: {tb}, Document: '{doc}', Reason: {}", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Deletes a key from a document in a table, 
    /// returns updated document or error if document, table, or key do not exist.
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
    ///     let key = "email";
    ///     db.put(tb, doc, key, json!("lucy@gmail.com")).await?;
    /// 
    ///     db.delete_key(tb, doc, key).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_key(&self, tb: &str, doc: &str, key: &str) -> Result<Value> {
        match self.delete_key_auth(tb, doc, key).await {
            Ok(res) => {
                commit_document(tb, doc, json!(res)).await?;
                log_delete_event(&f!("Table: '{tb}', Document: '{doc}', Key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Document: '{doc}', Key: '{key}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Deletes a document from a table, 
    /// returns deleted document or error if document or table do not exist.
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
    ///     db.merge(tb, doc, json!({"name": "Lucy"})).await?;
    /// 
    ///     db.delete_document(tb, doc).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_document(&self, tb: &str, doc: &str) -> Result<Value> {
        match self.delete_document_auth(tb, doc).await {
            Ok(res) => {
                delete_document(tb, doc).await?;
                log_delete_event(&f!("Table: '{tb}', Document: '{doc}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Document: '{doc}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Deletes a key from all documents in a table, 
    /// returns string confirmation or error if the table does not exist.
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
    ///     let key = "email"
    ///     db.delete_key_from_table(tb, key).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_key_from_table(&self, tb: &str, key: &str) -> Result<Value> {
        match self.delete_key_from_table_auth(tb, key).await {
            Ok(res) => {
                log_delete_event(&f!("REMOVED KEY >> Table: '{tb}', Key: '{key}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Key: '{key}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
    /// Deletes a table, returns string confirmation or error if the table does not exist.
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
    ///     let key = "email"
    ///     db.delete_key_from_table(tb, key).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_table(&self, tb: &str) -> Result<Value> {
        match self.delete_table_auth(tb).await {
            Ok(res) => {
                delete_table(tb).await?;
                log_delete_event(&f!("Table: '{tb}'")).await?;
                Ok(res)
            }
            Err(e) => {
                log_error(&f!("DELETE >> Table: '{tb}', Reason: '{}'", e.to_string())).await?;
                Err(e)
            }
        }
    }
}