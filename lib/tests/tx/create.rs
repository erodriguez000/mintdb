use mintdb::{kvs::store::Datastore, prelude::*};
use serde_json::json;

#[tokio::test]
async fn tx_creates() -> Result<()> {
    let db = Datastore::new().await?;
    let mut tx = db.transaction().await?;
    let tb = "tx_create";
    let doc = "tx_create:1";
    let data = json!({
        "name": "Lucy",
        "city": "Miami",
        "state": "FL"
    });

    tx.begin();
    tx.create_table(tb).await?;
    tx.commit().await?;

    tx.begin();
    tx.create_document(tb, doc, &data).await?;
    tx.commit().await?;

    let res = db.get_one_auth(tb, doc).await?;
    assert_eq!(res["name"], "Lucy");
    Ok(())
}

#[tokio::test]
async fn tx_creates_document_in_new_table() -> Result<()> {
    let db = Datastore::new().await?;
    let mut tx = db.transaction().await?;
    let tb = "tx_create";
    let doc = "tx_create:1";
    let data = json!({
        "name": "Lucy",
        "city": "Miami",
        "state": "FL"
    });

    tx.begin();
    tx.create_document(tb, doc, &data).await?;
    let res = tx.commit().await?;
    println!("{res}");

    let res = db.get_one_auth(tb, doc).await?;
    assert_eq!(res["name"], "Lucy");
    Ok(())
}

#[tokio::test]
async fn tx_create_c() -> Result<()> {
    let db = Datastore::new().await?;
    let mut tx = db.transaction().await?;
    let tb = "tx_create_c";
    let doc = "tx_create_c:1";
    let key = "key_to_delete";
    let data = json!({"key": "value", key: "value"});

    let doc2 = "tx_create_c:2";
    let data2 = json!({"key": "value", key: "value"});
    {
        tx.begin();
        tx.create_documentc(tb, doc, data).await?;
        tx.merge_c(tb, doc2, data2).await?;
        tx.commit_c().await?;

        tx.begin();
        tx.delete_keyc(tb, doc, key).await?;
        tx.delete_keyc(tb, doc2, key).await?;
        tx.commit_c().await?;
    }
    let res = db.get_one(tb, doc).await?;
    println!("{res}");

    let res = db.get_one(tb, doc2).await?;
    println!("{res}");
    Ok(())
}
