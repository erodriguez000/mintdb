<p align="center">
    <img width="400" src="../img/logo-light.svg#gh-light-mode-only" alt="mintDB-JS Logo">
    <img width="400" src="../img/logo.svg#gh-dark-mode-only" alt="mintDB-JS Logo">
</p>
<h2 align="center">An Open Source Graph Database</h2>
<p align="center">
    <img src="https://img.shields.io/badge/version-≈-10d99d">
    <img src="https://img.shields.io/badge/built_with-Rust-dca282.svg">
    <img src="https://img.shields.io/badge/license-MIT-critical">
    <a href="https://www.linkedin.com/in/eric-rodriguez-3a402811b/"><img src="https://img.shields.io/badge/linkedIn-connect-4777AF"></a>
</p>

# Getting Started

```
cargo add mintdb
```

or add the following to your Cargo.toml

```toml
mintdb = "0.1.0-beta.3
```
### Create a new Datastore instance

```rs
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let db = Datastore::new().await?;
    Ok(())
}
```

### CREATE

```rs
use serde_json::{Value, json};
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let tb = "db_create";
    let doc = "db_create:1";
    let data = json!({
        "name": "Lucy"
    });
    
    let db = Datastore::new().await?;

    // returns document or error if exists
    db.create_document(tb, doc, data).await?;

    let tb = "tb_create";

    // returns "OK" or error if exists
    db.create_tb(tb).await?;

    let res: Value = db.get_one(tb, doc).await?;
    Ok(())
}
```

### READ

```rs
use serde_json::{Value, json};
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let tb = "db_read";
    let doc = "db_read:1";

    let db = Datastore::new().await?;

    // returns a document or an Error if not found
    let res: Value = db.get_one(tb, doc).await?;

    // returns an array of all documents in the table
    let res: Value = db.get_many(tb).await?;

    // returns an array of documents matching any key value
    let search = json!({
        "name": "Lucy",
        "state": "FL"
    });
    let res: Value = db.find(tb, search).await?;

    // returns an array of documents matching all key values
    let search = json!({
        "name": "Lucy",
        "state": "FL"
    });
    let res: Value = db.match_all(tb, search).await?;

    // returns an array of documents matching the comparison
    // options for op: "==", "!=", ">=", ">", "<", "<=", "contains" (case sensitive), "icontains" (case insensitve)
    let tb = "car";
    let lhs = "model";
    let op = "icontains";
    let rhs = "amg";
    let res: Value = db.compare(tb, lhs, op, rhs).await?;

    Ok(())
}
```
### UPDATE

All create a new document and table if they do not exist

```rs
use serde_json::json;
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let tb = "db_update";
    let doc = "db_update:1";
    let data = json!({
        "name": "Lucy"
    });
    
    let db = Datastore::new().await?;

    // returns new document or error
    db.merge(tb, doc, data).await?;
    
    // returns new document or error
    let key = "name";
    let value = json!("Lucille");
    db.put(tb, doc, key, value).await?;

    // pushes a value to a key if it is an array, returns the document or error
    // returns an error if the key exists, but is not an array
    let key = "friends";
    let value = json!("Ricky");
    db.push(tb, doc, key, value).await?;

    Ok(())
}
```
### Delete

```rs
use serde_json::json;
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let tb = "db_delete";
    let doc = "db_delete:1";
    let data = json!({
        "name": "Lucy",
        "email": "lucy@gmail.com"
    });
    let db = Datastore::new().await?;

    db.create_document(tb, doc, data).await?;

    // returns document or error if document/table does not exist
    db.delete_key(tb, doc, "email").await?;

    // returns document or error if document/table does not exist
    db.delete_document(tb, doc).await?;

}
```
### Transaction

```rs
use serde_json::json;
use mintdb::Datastore;

[tokio::main]
async fn main() -> Result<(), mintdb::err::Error> {
    let db = Datastore::new().await?;

    let tb = "tx";
    let doc = "tx:1";
    let data = json!({
        "name": "Lucy",
        "balance": 1000.00 
    });

    db.create_document(tb, doc, data).await?;

    let tb = "tx";
    let doc = "tx:2";
    let data = json!({
        "name": "Ricky",
        "balance": 1000.00 
    });

    db.create_document(tb, doc, data).await?;

    let mut tx = db.transaction().await?;

    tx.begin();
    tx.debit("tx", "tx:2", "balance", 500.00).await?;
    tx.credit("tx", "tx:1", "balance", 500.00).await;
    tx.commit().await?;

}
```


