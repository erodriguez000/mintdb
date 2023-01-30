use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use mintdb::util::verify::check_key::document_exists;
use serde_json::json;
#[tokio::test]
async fn db_creates_document() -> Result<()> {
    let tb = "db_create";
    let doc = "db_create:1";
    let data = json!({
        "name": "Lucy"
    });
    
    let db = Datastore::new().await?;
    if document_exists(tb, doc) {
        db.delete_document(tb, doc).await?;
    }
    db.create_document(tb, doc, data).await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["name"], "Lucy");
    
    
    Ok(())
}

#[tokio::test]
async fn db_merges() -> Result<()> {
    let tb = "db_merge";
    let doc = "db_merge:1";
    let data = json!({"key": "value"});
    let db = Datastore::new().await?;
    let res = db.merge(tb, doc, data).await?;
    println!("{res}");
    
    Ok(())
}

#[tokio::test]
async fn db_pushes() -> Result<()> {
    let tb = "db_push";
    let doc = "db_push:1";
    let key = "array";
    let value = json!("Value");
    
    let db = Datastore::new().await?;
    
    db.push(tb, doc, key, value).await?;
    let res = db.get_one(tb, doc).await?;
    assert_eq!(res[key][0], "Value");
    
    let value = json!("Value");
    db.push(tb, doc, key, value).await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res[key][0], "Value");
    assert_eq!(res[key][1], "Value");
    
    Ok(())
}

#[tokio::test]
async fn db_puts() -> Result<()> {
    let tb = "db_push";
    let doc = "db_push:1";
    let key = "key";
    let value = "Value";
    let db = Datastore::new().await?;
    db.put(tb, doc, key, json!(value)).await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["key"], value);

    Ok(())
}