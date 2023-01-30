use serde_json::json;

use crate::fs::ds::write::commit_document;
use crate::kvs::store::Datastore;
use crate::prelude::*;

#[allow(unused)]
pub async fn dev_seed(db: &mut Datastore) -> Result<()>{
    match std::fs::metadata("mint.db") {
        Ok(_) => std::fs::remove_dir_all("mint.db")?,
        Err(_) => ()
    }
    // Person
    let tb = "person";
    let doc = "person:1";
    let data = json!({
        "name": "Eric",
        "city": "Tampa",
        "state": "FL",
        "email": "eric@gmail.com"
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;
    
    let doc = "person:2";
    let data = json!({
        "name": "Lucy",
        "city": "Miami",
        "state": "FL",
        "email": "lucy@gmail.com"
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;

    // Todo
    let tb = "todo";
    let doc = "todo:1";
    let data = json!({
        "author": "person:1",
        "title": "A Todo from Eric",
        "body": "Hello, world!"
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;

    let doc = "todo:2";
    let data = json!({
        "author": "person:1",
        "title": "Another Todo from Eric",
        "body": "Hello, world again!"
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;

    let doc = "todo:3";
    let data = json!({
        "author": "person:2",
        "title": "A Todo from Lucy",
        "body": "Wahhhh!"
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;

    // Car
    let tb = "car";
    let doc = "car:1";
    let data = json!({
        "make": "Mercedes-Benz",
        "model": "G-Wagon",
        "year": 2024
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;

    let doc = "car:2";
    let data = json!({
        "make": "Suzuki",
        "model": "Samauri",
        "year": 1995
    });
    db.create_document(tb, doc, data.clone()).await?;
    commit_document(tb, doc, data).await?;


    // db.register("eric@gmail.com", "abc123")?;
    // db.register("lucy@gmail.com", "abc123")?;

    Ok(())
}